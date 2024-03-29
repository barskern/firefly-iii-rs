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
pub struct CurrencyUpdate {
    /// If the currency is enabled
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// If the currency must be the default for the user. You can only submit TRUE.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<Default>,
    /// The currency code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The currency name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The currency symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// How many decimals to use when displaying this currency. Between 0 and 16.
    #[serde(rename = "decimal_places", skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
}

impl CurrencyUpdate {
    pub fn new() -> CurrencyUpdate {
        CurrencyUpdate {
            enabled: None,
            default: None,
            code: None,
            name: None,
            symbol: None,
            decimal_places: None,
        }
    }
}

/// If the currency must be the default for the user. You can only submit TRUE.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Default {
    #[serde(rename = "true")]
    _True,
}

impl std::default::Default for Default {
    fn default() -> Default {
        Self::_True
    }
}

