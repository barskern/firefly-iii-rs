/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */


use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicSummaryEntry {
    /// This is a reference to the type of info shared, not influenced by translations or user preferences.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// A translated title for the information shared.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The amount as a float.
    #[serde(rename = "monetary_value", skip_serializing_if = "Option::is_none")]
    pub monetary_value: Option<f64>,
    /// The currency ID of the associated currency.
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<i32>,
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "currency_symbol", skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    /// Number of decimals for the associated currency.
    #[serde(rename = "currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<i32>,
    /// The amount formatted according to the users locale
    #[serde(rename = "value_parsed", skip_serializing_if = "Option::is_none")]
    pub value_parsed: Option<String>,
    /// Reference to a font-awesome icon without the fa- part.
    #[serde(rename = "local_icon", skip_serializing_if = "Option::is_none")]
    pub local_icon: Option<String>,
    /// A short explanation of the amounts origin. Already formatted according to the locale of the user or translated, if relevant.
    #[serde(rename = "sub_title", skip_serializing_if = "Option::is_none")]
    pub sub_title: Option<String>,
}

impl BasicSummaryEntry {
    pub fn new() -> BasicSummaryEntry {
        BasicSummaryEntry {
            key: None,
            title: None,
            monetary_value: None,
            currency_id: None,
            currency_code: None,
            currency_symbol: None,
            currency_decimal_places: None,
            value_parsed: None,
            local_icon: None,
            sub_title: None,
        }
    }
}


