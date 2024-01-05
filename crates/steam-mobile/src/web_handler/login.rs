use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use chrono::Offset;
use const_format::concatcp;
use cookie::Cookie;
use futures_timer::Delay;
use parking_lot::RwLock;
use rand::thread_rng;
use reqwest::{Client, Method};
use rsa::padding::PaddingScheme;
use rsa::{BigUint, PublicKey, RsaPublicKey};
use steam_totp::{Secret, Time};

use crate::client::MobileClient;
use crate::errors::LoginError;
use crate::types::{resolve_login_response, LoginCaptcha, LoginRequest, RSAResponse};
use crate::{
    CachedInfo, User, MOBILE_REFERER, STEAM_COMMUNITY_BASE, STEAM_COMMUNITY_HOST, STEAM_DELAY_MS, STEAM_HELP_HOST,
    STEAM_STORE_HOST,
};

const LOGIN_GETRSA_URL: &str = concatcp!(STEAM_COMMUNITY_BASE, "/login/getrsakey");
const LOGIN_DO_URL: &str = concatcp!(STEAM_COMMUNITY_BASE, "/login/dologin");

type LoginResult<T> = Result<T, LoginError>;

/// This method is used to login through Steam `ISteamAuthUser` interface.
///
/// Webapi_nonce is received by connecting to the Steam Network.
///
/// Currently not possible without the implementation of the [steam-client] crate.
/// For website that currently works, check [login_website] method.
async fn login_isteam_user_auth(_client: &Client, _user: User, _webapi_nonce: &[u8]) -> LoginResult<()> {
    unimplemented!();
}

/// Website has some quirks to login. Here we handle it.
fn website_handle_rsa(user: &User, response: RSAResponse) -> String {
    let password_bytes = user.password.as_bytes();
    let modulus = hex::decode(response.modulus).unwrap();
    let exponent = hex::decode(response.exponent).unwrap();

    let rsa_encryptor = RsaPublicKey::new(BigUint::from_bytes_be(&modulus), BigUint::from_bytes_be(&exponent)).unwrap();
    let mut random_gen = thread_rng();
    let encrypted_pwd_bytes = rsa_encryptor
        .encrypt(&mut random_gen, PaddingScheme::PKCS1v15Encrypt, password_bytes)
        .unwrap();

    base64::encode(encrypted_pwd_bytes)
}

/// Used to login through the Steam Website.
/// Also caches the user steamid.
///
///
/// There is also the method that logs in through an API interface called ISteamUserAuth.
/// Check [login_isteam_user_auth]
///
/// https://github.com/Jessecar96/SteamBot/blob/e8e9e5fcd64ae35b201e2597068849c10a667b60/SteamTrade/SteamWeb.cs#L325
// We can really do that method yet, because connection to the SteamNetwork is not yet implemented
// by steam-client crate, and consequently we can't get the user webapi_nonce beforehand.
//
// Should accept closure to handle cases such as needing a captcha or sms.
// But the best way is to have it already setup to use TOTP codes.
pub(crate) async fn login_website<'a, LC>(
    client: &MobileClient,
    user: &User,
    cached_data: Arc<RwLock<CachedInfo>>,
    captcha: LC,
) -> LoginResult<()>
where
    LC: Into<Option<LoginCaptcha<'a>>>,
{
    // we request to generate sessionID cookies
    let response = client
        .request(MOBILE_REFERER.to_owned(), Method::GET, None, None::<&u8>)
        .await?;
    let session_id = response
        .headers()
        .get(reqwest::header::SET_COOKIE)
        .map(|cookie| cookie.to_str().unwrap())
        .map(|c| {
            let index = c.find('=').unwrap();
            c[index + 1..index + 25].to_string()
        })
        .ok_or_else(|| {
            LoginError::GeneralFailure("Something went wrong while getting sessionid. Should retry".to_string())
        })?;

    let mut post_data = HashMap::new();
    let steam_time_offset = (Time::offset().await? * 1000).to_string();
    post_data.insert("donotcache", &steam_time_offset);
    post_data.insert("username", &user.username);

    let rsa_response = client
        .request(LOGIN_GETRSA_URL.to_owned(), Method::POST, None, Some(&post_data))
        .await?;

    // wait for steam to catch up
    Delay::new(Duration::from_millis(STEAM_DELAY_MS)).await;

    // rsa handling
    let response = rsa_response
        .json::<RSAResponse>()
        .await
        .expect("There was an error deserializing RSA Response.");
    let encrypted_pwd_b64 = website_handle_rsa(user, response.clone());

    let offset = Time::offset().await?;
    let time = Time::now(Some(offset)).unwrap();

    let steam_time_offset = (offset * 1000).to_string();
    let two_factor_code = user
        .linked_mafile
        .as_ref()
        .map(|f| Secret::from_b64(&f.shared_secret).unwrap())
        .map_or_else(String::new, |s| steam_totp::generate_auth_code(s, time));

    let login_captcha = captcha.into();

    let login_request = LoginRequest {
        donotcache: &steam_time_offset,
        password: &encrypted_pwd_b64,
        username: &user.username,
        twofactorcode: &two_factor_code,
        emailauth: "",
        captcha_gid: login_captcha.as_ref().map_or_else(|| "-1", |x| x.guid),
        captcha_text: login_captcha.map_or_else(|| "", |x| x.text),
        emailsteamid: "",
        rsa_timestamp: response.timestamp,
        ..Default::default()
    };

    // This next operation will fail if called too fast, we should wait a bit.
    // time::delay_for(Duration::from_secs(2)).await;

    let login_response = client
        .request(LOGIN_DO_URL.to_owned(), Method::POST, None, Some(&login_request))
        .await?;

    let login_response_text = login_response.text().await?;
    let login_response_json = resolve_login_response(login_response_text)?;

    let steamid = login_response_json.oauth.steamid;
    let oauth_token = login_response_json.oauth.oauth_token;
    let token = login_response_json.oauth.wgtoken;
    let token_secure = login_response_json.oauth.wgtoken_secure;

    // cache steamid
    {
        let mut cached_data = cached_data.write();
        cached_data.set_steamid(&steamid);
        cached_data.set_oauth_token(oauth_token);
    }

    {
        // Recover cookies to authorize store.steampowered and help.steampowered subdomains.
        let mut cookie_jar = client.cookie_store.write();
        for host in &[STEAM_COMMUNITY_HOST, STEAM_HELP_HOST, STEAM_STORE_HOST] {
            let timezone_offset = format!("{},0", chrono::Local::now().offset().fix().local_minus_utc());
            let fmt_token = format!("{steamid}%7C%7C{token}");
            let fmt_secure_token = format!("{steamid}%7C%7C{token_secure}");
            cookie_jar.add_original(
                Cookie::build("steamLoginSecure", fmt_secure_token)
                    .domain(*host)
                    .path("/")
                    .finish(),
            );
            cookie_jar.add_original(Cookie::build("steamLogin", fmt_token).domain(*host).path("/").finish());
            cookie_jar.add_original(
                Cookie::build("sessionid", session_id.clone())
                    .domain(*host)
                    .path("/")
                    .finish(),
            );
            cookie_jar.add_original(
                Cookie::build("timezoneOffset", timezone_offset)
                    .domain(*host)
                    .path("/")
                    .finish(),
            );
        }
    }

    Ok(())
}
