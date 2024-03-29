/*
 * Firefly III API v1.5.5
 *
 * This is the documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. You may use the \"Authorize\" button to try the API below. This file was last generated on 2022-01-30T05:47:28+00:00 
 *
 * The version of the OpenAPI document: 1.5.5
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The new users email address.
    #[serde(rename = "email")]
    pub email: String,
    /// Boolean to indicate if the user is blocked.
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(rename = "blocked_code", skip_serializing_if = "Option::is_none")]
    pub blocked_code: Option<crate::models::UserBlockedCodeProperty>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::UserRoleProperty>,
}

impl User {
    pub fn new(email: String) -> User {
        User {
            created_at: None,
            updated_at: None,
            email,
            blocked: None,
            blocked_code: None,
            role: None,
        }
    }
}


