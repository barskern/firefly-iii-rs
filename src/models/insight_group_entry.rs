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
pub struct InsightGroupEntry {
    /// This ID is a reference to the original object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// This is the name of the object.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The amount spent or earned between start date and end date, a number defined as a string, for this object and all asset accounts.
    #[serde(rename = "difference", skip_serializing_if = "Option::is_none")]
    pub difference: Option<String>,
    /// The amount spent or earned between start date and end date, a number as a float, for this object and all asset accounts. May have rounding errors.
    #[serde(rename = "difference_float", skip_serializing_if = "Option::is_none")]
    pub difference_float: Option<f64>,
    /// The currency ID of the expenses listed for this account.
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// The currency code of the expenses listed for this account.
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

impl InsightGroupEntry {
    pub fn new() -> InsightGroupEntry {
        InsightGroupEntry {
            id: None,
            name: None,
            difference: None,
            difference_float: None,
            currency_id: None,
            currency_code: None,
        }
    }
}


