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
pub struct Configuration {
    #[serde(rename = "title")]
    pub title: crate::models::ConfigValueFilter,
    #[serde(rename = "value")]
    pub value: crate::models::PolymorphicProperty,
    /// If this config variable can be edited by the user
    #[serde(rename = "editable")]
    pub editable: bool,
}

impl Configuration {
    pub fn new(title: crate::models::ConfigValueFilter, value: crate::models::PolymorphicProperty, editable: bool) -> Configuration {
        Configuration {
            title,
            value: value,
            editable,
        }
    }
}


