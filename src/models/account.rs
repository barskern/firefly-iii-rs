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
pub struct Account {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// If omitted, defaults to true.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Order of the account. Is NULL if account is not asset or liability.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: crate::models::ShortAccountTypeProperty,
    #[serde(rename = "account_role", skip_serializing_if = "Option::is_none")]
    pub account_role: Option<crate::models::AccountRoleProperty>,
    /// Use either currency_id or currency_code. Defaults to the user's default currency.
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// Use either currency_id or currency_code. Defaults to the user's default currency.
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "currency_symbol", skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    #[serde(rename = "currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<i32>,
    #[serde(rename = "current_balance", skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<String>,
    #[serde(rename = "current_balance_date", skip_serializing_if = "Option::is_none")]
    pub current_balance_date: Option<String>,
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "bic", skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(rename = "account_number", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Represents the opening balance, the initial amount this account holds.
    #[serde(rename = "opening_balance", skip_serializing_if = "Option::is_none")]
    pub opening_balance: Option<String>,
    /// Represents the current debt for liabilities.
    #[serde(rename = "current_debt", skip_serializing_if = "Option::is_none")]
    pub current_debt: Option<String>,
    /// Represents the date of the opening balance.
    #[serde(rename = "opening_balance_date", skip_serializing_if = "Option::is_none")]
    pub opening_balance_date: Option<String>,
    #[serde(rename = "virtual_balance", skip_serializing_if = "Option::is_none")]
    pub virtual_balance: Option<String>,
    /// If omitted, defaults to true.
    #[serde(rename = "include_net_worth", skip_serializing_if = "Option::is_none")]
    pub include_net_worth: Option<bool>,
    #[serde(rename = "credit_card_type", skip_serializing_if = "Option::is_none")]
    pub credit_card_type: Option<crate::models::CreditCardType>,
    /// Mandatory when the account_role is ccAsset. Moment at which CC payment installments are asked for by the bank.
    #[serde(rename = "monthly_payment_date", skip_serializing_if = "Option::is_none")]
    pub monthly_payment_date: Option<String>,
    #[serde(rename = "liability_type", skip_serializing_if = "Option::is_none")]
    pub liability_type: Option<crate::models::LiabilityType>,
    #[serde(rename = "liability_direction", skip_serializing_if = "Option::is_none")]
    pub liability_direction: Option<crate::models::LiabilityDirection>,
    /// Mandatory when type is liability. Interest percentage.
    #[serde(rename = "interest", skip_serializing_if = "Option::is_none")]
    pub interest: Option<String>,
    #[serde(rename = "interest_period", skip_serializing_if = "Option::is_none")]
    pub interest_period: Option<crate::models::LiabilityDirection>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Latitude of the accounts's location, if applicable. Can be used to draw a map.
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// Latitude of the accounts's location, if applicable. Can be used to draw a map.
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Zoom level for the map, if drawn. This to set the box right. Unfortunately this is a proprietary value because each map provider has different zoom levels.
    #[serde(rename = "zoom_level", skip_serializing_if = "Option::is_none")]
    pub zoom_level: Option<i32>,
}

impl Account {
    pub fn new(name: String, _type: crate::models::ShortAccountTypeProperty) -> Account {
        Account {
            created_at: None,
            updated_at: None,
            active: None,
            order: None,
            name,
            _type,
            account_role: None,
            currency_id: None,
            currency_code: None,
            currency_symbol: None,
            currency_decimal_places: None,
            current_balance: None,
            current_balance_date: None,
            iban: None,
            bic: None,
            account_number: None,
            opening_balance: None,
            current_debt: None,
            opening_balance_date: None,
            virtual_balance: None,
            include_net_worth: None,
            credit_card_type: None,
            monthly_payment_date: None,
            liability_type: None,
            liability_direction: None,
            interest: None,
            interest_period: None,
            notes: None,
            latitude: None,
            longitude: None,
            zoom_level: None,
        }
    }
}


