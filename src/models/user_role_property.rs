/*
 * Firefly III API v1.5.5
 *
 * This is the documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. You may use the \"Authorize\" button to try the API below. This file was last generated on 2022-01-30T05:47:28+00:00 
 *
 * The version of the OpenAPI document: 1.5.5
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */

/// UserRoleProperty : Role for the user. Can be empty or omitted.

/// Role for the user. Can be empty or omitted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserRoleProperty {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "demo")]
    Demo,
    #[serde(rename = "null")]
    Null,

}

impl ToString for UserRoleProperty {
    fn to_string(&self) -> String {
        match self {
            Self::Owner => String::from("owner"),
            Self::Demo => String::from("demo"),
            Self::Null => String::from("null"),
        }
    }
}

impl std::default::Default for UserRoleProperty {
    fn default() -> UserRoleProperty {
        Self::Owner
    }
}




