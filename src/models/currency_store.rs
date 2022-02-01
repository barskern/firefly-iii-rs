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
pub struct CurrencyStore {
    /// Defaults to true
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Make this currency the default currency.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Supports 0-16 decimals.
    #[serde(rename = "decimal_places", skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
}

impl CurrencyStore {
    pub fn new(code: String, name: String, symbol: String) -> CurrencyStore {
        CurrencyStore {
            enabled: None,
            default: None,
            code,
            name,
            symbol,
            decimal_places: None,
        }
    }
}


