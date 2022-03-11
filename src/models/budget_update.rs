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
pub struct BudgetUpdate {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "auto_budget_type", skip_serializing_if = "Option::is_none")]
    pub auto_budget_type: Option<crate::models::AutoBudgetType>,
    /// Use either currency_id or currency_code. Defaults to the user's default currency.
    #[serde(rename = "auto_budget_currency_id", skip_serializing_if = "Option::is_none")]
    pub auto_budget_currency_id: Option<String>,
    /// Use either currency_id or currency_code. Defaults to the user's default currency.
    #[serde(rename = "auto_budget_currency_code", skip_serializing_if = "Option::is_none")]
    pub auto_budget_currency_code: Option<String>,
    #[serde(rename = "auto_budget_amount", skip_serializing_if = "Option::is_none")]
    pub auto_budget_amount: Option<String>,
    #[serde(rename = "auto_budget_period", skip_serializing_if = "Option::is_none")]
    pub auto_budget_period: Option<crate::models::AutoBudgetPeriod>,
}

impl BudgetUpdate {
    pub fn new() -> BudgetUpdate {
        BudgetUpdate {
            name: None,
            active: None,
            order: None,
            auto_budget_type: None,
            auto_budget_currency_id: None,
            auto_budget_currency_code: None,
            auto_budget_amount: None,
            auto_budget_period: None,
        }
    }
}


