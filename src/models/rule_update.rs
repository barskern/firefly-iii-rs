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
pub struct RuleUpdate {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID of the rule group under which the rule must be stored. Either this field or rule_group_title is mandatory.
    #[serde(rename = "rule_group_id", skip_serializing_if = "Option::is_none")]
    pub rule_group_id: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<crate::models::RuleTriggerType>,
    /// Whether or not the rule is even active. Default is true.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// If the rule is set to be strict, ALL triggers must hit in order for the rule to fire. Otherwise, just one is enough. Default value is true.
    #[serde(rename = "strict", skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    /// If this value is true and the rule is triggered, other rules  after this one in the group will be skipped. Default value is false.
    #[serde(rename = "stop_processing", skip_serializing_if = "Option::is_none")]
    pub stop_processing: Option<bool>,
    #[serde(rename = "triggers", skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<crate::models::RuleTriggerUpdate>>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<crate::models::RuleActionUpdate>>,
}

impl RuleUpdate {
    pub fn new() -> RuleUpdate {
        RuleUpdate {
            title: None,
            description: None,
            rule_group_id: None,
            order: None,
            trigger: None,
            active: None,
            strict: None,
            stop_processing: None,
            triggers: None,
            actions: None,
        }
    }
}


