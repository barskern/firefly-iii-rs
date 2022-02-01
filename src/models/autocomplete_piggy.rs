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
pub struct AutocompletePiggy {
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the piggy bank found by an auto-complete search.
    #[serde(rename = "name")]
    pub name: String,
    /// Currency ID for this piggy bank.
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// Currency code for this piggy bank.
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "currency_symbol", skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    /// Currency name for the currency used by this account.
    #[serde(rename = "currency_name", skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<String>,
    #[serde(rename = "currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<i32>,
    /// The group ID of the group this object is part of. NULL if no group.
    #[serde(rename = "object_group_id", skip_serializing_if = "Option::is_none")]
    pub object_group_id: Option<String>,
    /// The name of the group. NULL if no group.
    #[serde(rename = "object_group_title", skip_serializing_if = "Option::is_none")]
    pub object_group_title: Option<String>,
}

impl AutocompletePiggy {
    pub fn new(id: String, name: String) -> AutocompletePiggy {
        AutocompletePiggy {
            id,
            name,
            currency_id: None,
            currency_code: None,
            currency_symbol: None,
            currency_name: None,
            currency_decimal_places: None,
            object_group_id: None,
            object_group_title: None,
        }
    }
}


