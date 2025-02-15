//! Contains structs and enums related to the x/reactions module.

use crate::reactions::models::UnwrapReactionValueError::{
    InvalidFreeTextValue, InvalidRegisteredReactionValue,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};
use std::convert::TryFrom;
use thiserror::Error;

/// Contains the data of a single post reaction.
#[cw_serde]
pub struct Reaction {
    /// Id of the subspace inside which the reaction has been put.
    pub subspace_id: Uint64,
    /// Id of the post to which the reaction is associated.
    pub post_id: Uint64,
    /// Id of the reaction within the post.
    pub id: u32,
    /// Value of the reaction.
    pub value: RawReactionValue,
    /// Author of the reaction.
    pub author: Addr,
}

/// Proto type uri of [`ReactionValue::Registered`].
pub const REGISTERED_REACTION_VALUE_TYPE_URI: &str = "/desmos.reactions.v1.RegisteredReactionValue";
/// Proto type uri of [`ReactionValue::FreeText`].
pub const FREE_TEXT_VALUE_TYPE_URI: &str = "/desmos.reactions.v1.FreeTextValue";

/// Struct representing a generic reaction value that can be serialized and sent to the chain.
/// This struct can be created converting a [`ReactionValue`] using the [`core::convert::Into`] trait.
#[cw_serde]
pub struct RawReactionValue {
    /// ReactionValue uri type, can be:
    /// * `/desmos.reactions.v1.RegisteredReactionValue` if representing a registered reaction.
    /// * `/desmos.reactions.v1.FreeTextValue` if representing a free text reaction.
    #[serde(rename = "@type")]
    type_uri: String,
    /// Represents the id of registered reaction if it is registered reaction reaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    registered_reaction_id: Option<u32>,
    /// Represents the text of reaction if it is free text reaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

/// Supported reaction value that can be added to a post.
#[cw_serde]
pub enum ReactionValue {
    /// Represents the registered reaction.
    Registered {
        /// Id of the registered reaction.
        registered_reaction_id: u32,
    },
    /// Represents the free text reaction.
    FreeText {
        /// Text of the reaction value.
        text: String,
    },
}

impl From<ReactionValue> for RawReactionValue {
    fn from(reaction: ReactionValue) -> Self {
        match reaction {
            ReactionValue::Registered {
                registered_reaction_id,
            } => RawReactionValue {
                type_uri: REGISTERED_REACTION_VALUE_TYPE_URI.to_string(),
                registered_reaction_id: Some(registered_reaction_id),
                text: None,
            },
            ReactionValue::FreeText { text } => RawReactionValue {
                type_uri: FREE_TEXT_VALUE_TYPE_URI.to_string(),
                registered_reaction_id: None,
                text: Some(text),
            },
        }
    }
}

/// Represents the errors that can occur when converting a [`RawReactionValue`] into a [`ReactionValue`].
#[derive(Error, Debug, Clone, PartialEq)]
pub enum UnwrapReactionValueError {
    /// Error that occur if [`RawReactionValue`] have an unknown reaction value type.
    #[error("unknown reaction value type: {0}")]
    UnknownReactionValue(String),
    /// Error that occur if [`RawReactionValue`] have type `/desmos.reactions.v1.RegisteredReactionValue` but
    /// some fields are undefined.
    #[error("invalid registered reaction value field {0} is none")]
    InvalidRegisteredReactionValue(String),
    /// Error that occur if [`RawReactionValue`] have type `desmos.reactions.v1.FreeTextValue` but
    /// some fields are undefined.
    #[error("invalid free text vlue field {0} is none")]
    InvalidFreeTextValue(String),
}

impl TryFrom<RawReactionValue> for ReactionValue {
    type Error = UnwrapReactionValueError;

    fn try_from(value: RawReactionValue) -> Result<Self, Self::Error> {
        if value.type_uri == REGISTERED_REACTION_VALUE_TYPE_URI {
            Ok(ReactionValue::Registered {
                registered_reaction_id: value.registered_reaction_id.ok_or_else(|| {
                    InvalidRegisteredReactionValue("registered_reaction_id".to_string())
                })?,
            })
        } else if value.type_uri == FREE_TEXT_VALUE_TYPE_URI {
            Ok(ReactionValue::FreeText {
                text: value
                    .text
                    .ok_or_else(|| InvalidFreeTextValue("text".to_string()))?,
            })
        } else {
            Err(UnwrapReactionValueError::UnknownReactionValue(
                value.type_uri,
            ))
        }
    }
}

/// Contains the details of a registered reaction within a subspace.
#[cw_serde]
pub struct RegisteredReaction {
    /// Id of the subspace for which this reaction has been registered.
    pub subspace_id: Uint64,
    /// Id of the registered reaction.
    pub id: u32,
    /// Unique shorthand code associated to this reaction.
    pub shorthand_code: String,
    /// Value that should be displayed when using this reaction.
    pub display_value: String,
}

/// Contains the parameters related to a single subspace reactions.
#[cw_serde]
pub struct SubspaceReactionsParams {
    /// Id of the subspace for which these params are valid.
    pub subspace_id: Uint64,
    /// Params related to [`ReactionValue::Registered`] reactions.
    pub registered_reaction: RegisteredReactionValueParams,
    /// Params related to [`ReactionValue::FreeText`] reactions.
    pub free_text: FreeTextValueParams,
}

/// Contains the params for [`ReactionValue::FreeText`] based reactions.
#[cw_serde]
pub struct FreeTextValueParams {
    /// Whether [`ReactionValue::FreeText`] reactions should be enabled.
    pub enabled: bool,
    /// The max length that [`ReactionValue::FreeText`] reactions should have.
    pub max_length: u32,
    /// RegEx that each [`ReactionValue::FreeText`] should respect.
    /// This is useful to limit what characters can be used as a reaction.
    pub reg_ex: String,
}

/// Contains the params for [`ReactionValue::Registered`] based reactions.
#[cw_serde]
pub struct RegisteredReactionValueParams {
    /// Whether [`ReactionValue::Registered`] reactions should be enabled.
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reaction_value_try_from_raw_with_invalid_type_uri_error() {
        let raw = RawReactionValue {
            type_uri: "/desmos.reactions.v1.InvalidValue".to_string(),
            registered_reaction_id: None,
            text: None,
        };
        assert_eq!(
            UnwrapReactionValueError::UnknownReactionValue(
                "/desmos.reactions.v1.InvalidValue".to_string()
            ),
            ReactionValue::try_from(raw).unwrap_err()
        )
    }
    #[test]
    fn registered_reaction_value_try_from_wrong_raw_error() {
        let raw = RawReactionValue {
            type_uri: "/desmos.reactions.v1.RegisteredReactionValue".to_string(),
            registered_reaction_id: None,
            text: None,
        };
        assert_eq!(
            UnwrapReactionValueError::InvalidRegisteredReactionValue(
                "registered_reaction_id".to_string()
            ),
            ReactionValue::try_from(raw).unwrap_err()
        )
    }
    #[test]
    fn registered_reaction_value_try_from_raw_properly() {
        let raw = RawReactionValue {
            type_uri: "/desmos.reactions.v1.RegisteredReactionValue".to_string(),
            registered_reaction_id: Some(1),
            text: None,
        };
        assert_eq!(
            ReactionValue::Registered {
                registered_reaction_id: 1
            },
            ReactionValue::try_from(raw).unwrap()
        )
    }
    #[test]
    fn free_text_value_try_from_wrong_raw_error() {
        let raw = RawReactionValue {
            type_uri: "/desmos.reactions.v1.FreeTextValue".to_string(),
            registered_reaction_id: None,
            text: None,
        };
        assert_eq!(
            UnwrapReactionValueError::InvalidFreeTextValue("text".to_string()),
            ReactionValue::try_from(raw).unwrap_err()
        )
    }
    #[test]
    fn free_text_reaction_value_try_from_raw_properly() {
        let raw = RawReactionValue {
            type_uri: "/desmos.reactions.v1.FreeTextValue".to_string(),
            registered_reaction_id: None,
            text: Some("test".to_string()),
        };
        assert_eq!(
            ReactionValue::FreeText {
                text: "test".to_string()
            },
            ReactionValue::try_from(raw).unwrap()
        )
    }
}
