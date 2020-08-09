/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */


use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BudgetSpent {
    /// The amount spent.
    #[serde(rename = "sum", skip_serializing_if = "Option::is_none")]
    pub sum: Option<String>,
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<i32>,
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "currency_symbol", skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    /// Number of decimals supported by the currency
    #[serde(rename = "currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<i32>,
}

impl BudgetSpent {
    pub fn new() -> BudgetSpent {
        BudgetSpent {
            sum: None,
            currency_id: None,
            currency_code: None,
            currency_symbol: None,
            currency_decimal_places: None,
        }
    }
}


