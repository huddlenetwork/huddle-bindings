//! Contains structs and enums related to the x/reports module.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};
use std::convert::TryFrom;
use thiserror::Error;

const USER_REPORT_TARGET_TYPE_URI: &str = "/desmos.reports.v1.UserTarget";
const POST_REPORT_TARGET_TYPE_URI: &str = "/desmos.reports.v1.PostTarget";

/// Represents a generic report.
#[cw_serde]
pub struct Report {
    /// Id of the subspace for which the report has been created.
    pub subspace_id: Uint64,
    /// Id of the report.
    pub id: Uint64,
    /// Id of the reason this report has been created for.
    pub reasons_ids: Vec<u32>,
    /// Message attached to this report.
    pub message: Option<String>,
    /// Address of the reporter.
    pub reporter: Addr,
    /// Target of the report.
    pub target: RawReportTarget,
    /// Time in which the report was created in RFC 3339 format.
    /// example: 1972-01-01T10:00:20.
    pub creation_date: String,
}

/// Serializable representation of [`ReportTarget`].  
/// To create an instance of this struct use the `into()` method of [`ReportTarget`]
#[cw_serde]
pub struct RawReportTarget {
    #[serde(rename = "@type")]
    type_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<Addr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_id: Option<Uint64>,
}

/// Types of target for which a report can be made.
#[derive(Clone, Debug, PartialEq)]
pub enum ReportTarget {
    /// Represents a report about a user.
    User {
        /// Address of the reported user.
        user: Addr,
    },
    /// Represents a report about a post.
    Post {
        /// Id of the reported post.
        post_id: Uint64,
    },
}

/// Contains the data about a reporting reason.
#[cw_serde]
pub struct Reason {
    /// Id of the subspace for which this reason is valid.
    pub subspace_id: Uint64,
    /// Id of the reason inside the subspace.
    pub id: u32,
    /// Title of the reason.
    pub title: String,
    /// Extended description of the reason and the cases it applies to.
    pub description: Option<String>,
}

impl From<ReportTarget> for RawReportTarget {
    fn from(target: ReportTarget) -> Self {
        match target {
            ReportTarget::User { user } => RawReportTarget {
                type_uri: USER_REPORT_TARGET_TYPE_URI.to_string(),
                user: Some(user),
                post_id: None,
            },
            ReportTarget::Post { post_id } => RawReportTarget {
                type_uri: POST_REPORT_TARGET_TYPE_URI.to_string(),
                user: None,
                post_id: Some(post_id),
            },
        }
    }
}

/// Represents the errors that can occur when converting a [`RawReportTarget`] into a [`ReportTarget`].
#[derive(Error, Debug, Clone, PartialEq)]
pub enum UnwrapReportTargetError {
    /// Error that occur if [`RawReportTarget`] has an unknown type.
    #[error("unknown attachment type: {0}")]
    UnknownType(String),
    /// Error that occur if [`RawReportTarget`] has type `/desmos.reports.v1.UserTarget` but
    /// one one field is undefined.
    #[error("invalid user target report field {0} is none")]
    InvalidUserTarget(String),
    /// Error that occur if [`RawReportTarget`] has type `/desmos.reports.v1.PostTarget` but
    /// one field is undefined.
    #[error("invalid post target report field {0} is none")]
    InvalidPostTarget(String),
}

impl TryFrom<RawReportTarget> for ReportTarget {
    type Error = UnwrapReportTargetError;

    fn try_from(value: RawReportTarget) -> Result<Self, Self::Error> {
        match value.type_uri.as_str() {
            USER_REPORT_TARGET_TYPE_URI => Ok(ReportTarget::User {
                user: Addr::unchecked(value.user.ok_or_else(|| {
                    UnwrapReportTargetError::InvalidUserTarget("user".to_string())
                })?),
            }),
            POST_REPORT_TARGET_TYPE_URI => Ok(ReportTarget::Post {
                post_id: value.post_id.ok_or_else(|| {
                    UnwrapReportTargetError::InvalidPostTarget("post_id".to_string())
                })?,
            }),
            _ => Err(UnwrapReportTargetError::UnknownType(value.type_uri)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn report_target_try_from_raw_with_invalid_type_uri_error() {
        let raw = RawReportTarget {
            type_uri: "/desmos.reports.v1.InvalidTarget".to_string(),
            user: None,
            post_id: None,
        };
        assert_eq!(
            UnwrapReportTargetError::UnknownType("/desmos.reports.v1.InvalidTarget".to_string()),
            ReportTarget::try_from(raw).unwrap_err()
        )
    }
    #[test]
    fn user_report_target_try_from_wrong_raw_error() {
        let raw = RawReportTarget {
            type_uri: "/desmos.reports.v1.UserTarget".to_string(),
            user: None,
            post_id: None,
        };
        assert_eq!(
            UnwrapReportTargetError::InvalidUserTarget("user".to_string()),
            ReportTarget::try_from(raw).unwrap_err()
        )
    }
    #[test]
    fn user_report_target_try_from_raw_properly() {
        let raw = RawReportTarget {
            type_uri: "/desmos.reports.v1.UserTarget".to_string(),
            user: Some(Addr::unchecked("user")),
            post_id: None,
        };
        assert_eq!(
            ReportTarget::User {
                user: Addr::unchecked("user")
            },
            ReportTarget::try_from(raw).unwrap()
        )
    }
    #[test]
    fn post_report_target_try_from_wrong_raw_error() {
        let raw = RawReportTarget {
            type_uri: "/desmos.reports.v1.PostTarget".to_string(),
            user: None,
            post_id: None,
        };
        assert_eq!(
            UnwrapReportTargetError::InvalidPostTarget("post_id".to_string()),
            ReportTarget::try_from(raw).unwrap_err()
        )
    }
    #[test]
    fn post_report_target_try_from_raw_properly() {
        let raw = RawReportTarget {
            type_uri: "/desmos.reports.v1.PostTarget".to_string(),
            user: None,
            post_id: Some(Uint64::new(1)),
        };
        assert_eq!(
            ReportTarget::Post {
                post_id: Uint64::new(1)
            },
            ReportTarget::try_from(raw).unwrap()
        )
    }
}
