use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use steam_language_gen::generated::enums::EResult;

use crate::errors::LoginError;

/// Used to login into Steam website if it detects something different on your account.
/// This may be because of unsuccessful logins, numerous retries on some operations. or anything. Really.
/// Requiring a captcha certainly can occur on an account with SteamGuard disabled, but still not sure on an account
/// with it disabled.
///
/// The safe way to use this if you are automating something, is to match the error and perhaps have a captcha solver.
/// You can get the captcha GUID if it is required through the `LoginError::CaptchaRequired`.
#[derive(Serialize, Debug, Clone)]
pub struct LoginCaptcha<'a> {
    #[serde(rename = "captcha_gid")]
    /// Captcha GUID. I.e unique identifier.
    pub guid: &'a str,
    #[serde(rename = "captcha_text")]
    /// Text transcription of captcha,
    pub text: &'a str,
}

#[derive(Serialize, Debug, Clone)]
pub struct ConfirmationMultiAcceptRequest<'a> {
    #[serde(rename = "a")]
    pub steamid: &'a str,
    #[serde(rename = "k")]
    pub confirmation_hash: String,
    #[serde(rename = "m")]
    pub device_kind: &'a str,
    #[serde(rename = "op")]
    /// Accept or cancel confirmation
    pub operation: &'a str,
    #[serde(rename = "p")]
    pub device_id: &'a str,
    #[serde(rename = "t")]
    pub time: &'a str,
    pub tag: &'a str,
    #[serde(flatten, with = "serde_with::rust::tuple_list_as_map")]
    pub confirmation_id: Vec<(&'a str, String)>,
    #[serde(flatten, with = "serde_with::rust::tuple_list_as_map")]
    pub confirmation_key: Vec<(&'a str, String)>,
}

impl<'a> Default for ConfirmationMultiAcceptRequest<'a> {
    fn default() -> Self {
        Self {
            steamid: "",
            confirmation_hash: String::new(),
            device_kind: "android",
            operation: "",
            device_id: "",
            time: "",
            tag: "conf",
            confirmation_id: vec![],
            confirmation_key: vec![],
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfirmationDetailsResponse {
    success: bool,
    pub html: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ParentalUnlockResponse {
    pub success: bool,
    pub eresult: EResult,
    pub error_message: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BooleanResponse {
    pub success: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct ParentalUnlockRequest<'a> {
    /// Parental Unlock Code
    pub pin: &'a str,
    pub sessionid: &'a str,
}

#[derive(Serialize, Debug, Clone)]
pub struct IEconServiceGetTradeOffersRequest {
    pub active_only: u8,
    pub get_descriptions: u8,
    pub get_sent_offers: u8,
    pub get_received_offers: u8,
    #[serde(rename = "key")]
    pub api_key: String,
    pub time_historical_cutoff: u32,
}

impl Default for IEconServiceGetTradeOffersRequest {
    fn default() -> Self {
        Self {
            active_only: 1,
            get_descriptions: 1,
            get_received_offers: 1,
            get_sent_offers: 0,
            api_key: "".to_string(),
            time_historical_cutoff: u32::max_value(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RSAResponse {
    success: bool,
    #[serde(rename = "publickey_exp")]
    pub exponent: String,
    #[serde(rename = "publickey_mod")]
    pub modulus: String,
    pub timestamp: String,
    token_gid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest<'a> {
    pub donotcache: &'a str,
    pub password: &'a str,
    pub username: &'a str,
    pub twofactorcode: &'a str,
    pub emailauth: &'a str,
    pub loginfriendlyname: &'a str,
    #[serde(rename = "captchagid")]
    pub captcha_gid: &'a str,
    pub captcha_text: &'a str,
    pub emailsteamid: &'a str,
    #[serde(rename = "rsatimestamp")]
    pub rsa_timestamp: String,
    pub remember_login: &'a str,
    pub oauth_client_id: &'a str,
    pub oauth_score: &'a str,
}

impl<'a> Default for LoginRequest<'a> {
    fn default() -> Self {
        Self {
            donotcache: "",
            password: "",
            username: "",
            twofactorcode: "",
            emailauth: "",
            loginfriendlyname: "",
            captcha_gid: "-1",
            captcha_text: "",
            emailsteamid: "",
            rsa_timestamp: "".to_string(),
            remember_login: "false",
            oauth_client_id: "DE45CD61",
            oauth_score: "read_profile write_profile read_client write_client",
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub requires_twofactor: bool,
    pub login_complete: bool,
    pub transfer_urls: Vec<String>,
    pub transfer_parameters: TransferParameters,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TransferParameters {
    pub steamid: String,
    pub token_secure: String,
    pub auth: String,
    pub remember_login: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginErrorGenericMessage<'a> {
    pub success: bool,
    pub message: Cow<'a, str>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginErrorCaptcha<'a> {
    pub success: bool,
    pub message: Cow<'a, str>,
    pub captcha_needed: bool,
    pub captcha_gid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LoginResponseMobile {
    pub success: bool,
    pub requires_twofactor: bool,
    pub redirect_uri: String,
    pub login_complete: bool,
    #[serde(deserialize_with = "serde_with::json::nested::deserialize")]
    pub oauth: Oauth,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Oauth {
    pub steamid: String,
    pub account_name: String,
    /// This is also known as "access_token", and can be used to refresh sessions.
    pub oauth_token: String,
    pub wgtoken: String,
    pub wgtoken_secure: String,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyRegisterRequest<'a> {
    #[serde(rename = "agreeToTerms")]
    agree_to_terms: &'a str,
    domain: &'a str,
    #[serde(rename = "Submit")]
    submit: &'a str,
}

impl<'a> Default for ApiKeyRegisterRequest<'a> {
    fn default() -> Self {
        Self {
            agree_to_terms: "agreed",
            domain: "localhost",
            submit: "Register",
        }
    }
}

#[derive(Deserialize)]
pub struct ISteamUserAuthResponse {
    token: String,
    #[serde(rename = "tokensecure")]
    token_secure: String,
}

#[derive(Serialize)]
pub struct ISteamUserAuthRequest {
    pub steamid: String,
    #[serde(rename = "sessionkey")]
    pub session_key: String,
    pub encrypted_loginkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveVanityUrlBaseResponse {
    pub response: ResolveVanityUrlResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveVanityUrlResponse {
    pub steamid: String,
    pub success: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveVanityUrlRequest {
    #[serde(rename = "key")]
    api_key: String,
    #[serde(rename = "vanityurl")]
    vanity_url: String,
}

pub fn resolve_login_response(response_text: String) -> Result<LoginResponseMobile, LoginError> {
    if let Ok(login_resp) = serde_json::from_str::<LoginResponseMobile>(&*response_text) {
        Ok(login_resp)
    } else {
        // checks for captcha error
        if let Ok(res) = serde_json::from_str::<LoginErrorCaptcha>(&*response_text) {
            tracing::warn!("Captcha is required.");
            return Err(LoginError::CaptchaRequired {
                captcha_guid: res.captcha_gid,
            });
        }

        if response_text.contains("account name or password that you have entered is incorrect") {
            return Err(LoginError::IncorrectCredentials);
        }

        tracing::warn!("Generic error {:?}", response_text);
        Err(LoginError::GeneralFailure(response_text))
    }
}
