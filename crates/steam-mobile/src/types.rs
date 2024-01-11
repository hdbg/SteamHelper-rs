use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use steam_language_gen::generated::enums::EResult;

use crate::STEAM_COMMUNITY_BASE;

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginErrorGenericMessage<'a> {
    pub success: bool,
    pub message: Cow<'a, str>,
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

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RSAResponseBase {
    #[serde(rename = "response")]
    pub inner: RSAResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RSAResponse {
    pub publickey_mod: String,
    pub publickey_exp: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
struct AjaxRefreshResponse {
    success: bool,
    login_url: String, // settoken url
    #[serde(rename = "steamID")]
    steam_id: String,
    nonce: String,
    redit: String,
    auth: String,
}

/// Request containing data coming from AjaxRefresh
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SetTokenRequest {
    #[serde(rename = "steamID")]
    steam_id: String,
    nonce: String,
    redir: String,
    auth: String,
}

#[derive(Debug, Deserialize)]
struct SetTokenResponse {
    response: i32, // 1 if ok
}

#[derive(Debug, Serialize)]
pub struct FinalizeLoginRequest {
    nonce: String,
    #[serde(rename = "sessionid")]
    session_id: String,
    redir: String,
}

impl FinalizeLoginRequest {
    pub(crate) fn new(refresh_token: String, session_id: String) -> Self {
        Self {
            nonce: refresh_token,
            session_id,
            redir: STEAM_COMMUNITY_BASE.to_owned() + "/login/home?goto=",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FinalizeLoginResponseBase {
    #[serde(rename = "steamID")]
    pub(crate) steam_id: String,
    redir: String,
    #[serde(rename = "transfer_info")]
    pub(crate) domain_tokens: Vec<DomainToken>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainToken {
    /// One of Steam's domains to ping and receive cookies for.
    pub url: String,
    pub params: DomainTokenData,
}

/// Contains tokens to authenticate into a Steam domain.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainTokenData {
    pub nonce: String,
    pub auth: String,
    #[serde(rename = "steamID")]
    pub steam_id: Option<String>,
}
