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
pub struct RuleActionStore {
    #[serde(rename = "type")]
    pub _type: crate::models::RuleActionKeyword,
    /// The accompanying value the action will set, change or update. Can be empty, but for some types this value is mandatory.
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// Order of the action
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// If the action is active. Defaults to true.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// When true, other actions will not be fired after this action has fired. Defaults to false.
    #[serde(rename = "stop_processing", skip_serializing_if = "Option::is_none")]
    pub stop_processing: Option<bool>,
}

impl RuleActionStore {
    pub fn new(_type: crate::models::RuleActionKeyword, value: Option<String>) -> RuleActionStore {
        RuleActionStore {
            _type,
            value,
            order: None,
            active: None,
            stop_processing: None,
        }
    }
}

