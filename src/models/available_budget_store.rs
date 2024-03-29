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
pub struct AvailableBudgetStore {
    /// Use either currency_id or currency_code.
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// Use either currency_id or currency_code.
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "amount")]
    pub amount: String,
    /// Start date of the available budget.
    #[serde(rename = "start")]
    pub start: String,
    /// End date of the available budget.
    #[serde(rename = "end")]
    pub end: String,
}

impl AvailableBudgetStore {
    pub fn new(amount: String, start: String, end: String) -> AvailableBudgetStore {
        AvailableBudgetStore {
            currency_id: None,
            currency_code: None,
            amount,
            start,
            end,
        }
    }
}


