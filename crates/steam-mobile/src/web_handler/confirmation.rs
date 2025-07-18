use std::fmt::Display;
use std::fmt::Formatter;
use std::iter::FromIterator;

use derive_more::Deref;
use derive_more::IntoIterator;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

/// A collection of [`Confirmation`]
#[derive(IntoIterator, Deref, Default, Debug)]
pub struct Confirmations(#[into_iterator(owned, ref)] pub Vec<Confirmation>);

impl<'a> FromIterator<&'a Confirmation> for Confirmations {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = &'a Confirmation>,
    {
        let buffer = iter.into_iter().cloned().collect::<Vec<_>>();
        Self(buffer)
    }
}

/// A pending Steam confirmation.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Confirmation {
    pub id: String,
    #[serde(rename = "nonce")]
    pub key: String,
    #[serde(rename = "type")]
    pub kind: EConfirmationType,
    pub creation_time: i64,
    pub creator_id: String,
    pub type_name: String,
    // from below here, nothing really useful
    // pub cancel: String,
    // pub accept: String,
    // pub icon: String,
    // pub multi: bool,
    // pub headline: String,
    // pub summary: Vec<String>,
    // pub warn: Option<String>,
}

impl Display for Confirmation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Confirmation {} of {:?}", self.key, self.kind)
    }
}

impl Confirmation {
    pub fn has_trade_offer_id(&self, offer_id: u64) -> bool {
        self.kind == EConfirmationType::Trade && offer_id == self.creator_id.parse::<u64>().unwrap()
    }
    pub fn trade_offer_id(&self) -> Option<u64> {
        if self.kind == EConfirmationType::Trade {
            self.creator_id.parse().ok()
        } else {
            None
        }
    }
}

/// We retrieve [`ConfirmationDetails`] as a json object.
/// There is also the need to already have a [Confirmation].
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct ConfirmationDetails {
    /// ID of the trade offer. Has a value if EConfirmationType::Trade
    pub trade_offer_id: Option<i64>,
}

/// Kinds of mobile confirmations
#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum EConfirmationType {
    /// Unknown confirmation
    Unknown = 0,
    /// Under rare circumstances this might pop up
    Generic = 1,
    /// Confirmation from Trade Offer
    Trade = 2,
    /// Confirmation from Steam's Market
    Market = 3,

    /// Unknown
    FeatureOptOut = 4,
    /// Confirmation for a phone number change
    PhoneNumberChange = 5,
    /// Confirmation for account recovery
    AccountRecovery = 6,
    /// Confirmation for creating a new API Key,
    APIKey = 9,
}

impl From<Vec<Confirmation>> for Confirmations {
    fn from(confirmations_vec: Vec<Confirmation>) -> Self {
        Self(confirmations_vec)
    }
}

#[allow(missing_docs)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ConfirmationAction {
    Retrieve,
    Accept,
    Deny,
}

impl ConfirmationAction {
    pub(crate) const fn as_operation(self) -> Option<&'static str> {
        Some(match self {
            Self::Accept => "allow",
            Self::Deny => "cancel",
            _ => return None,
        })
    }
    pub(crate) const fn as_tag(self) -> &'static str {
        "conf"
    }
}

#[derive(Copy, Clone, Debug)]
enum EInventoryPrivacy {
    Unknown,
    Private,
    FriendsOnly,
    Public,
}