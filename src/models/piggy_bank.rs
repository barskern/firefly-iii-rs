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
pub struct PiggyBank {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The ID of the asset account this piggy bank is connected to.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The name of the asset account this piggy bank is connected to.
    #[serde(rename = "account_name", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "currency_symbol", skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    /// Number of decimals supported by the currency
    #[serde(rename = "currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<i32>,
    #[serde(rename = "target_amount")]
    pub target_amount: String,
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f32>,
    #[serde(rename = "current_amount", skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<String>,
    #[serde(rename = "left_to_save", skip_serializing_if = "Option::is_none")]
    pub left_to_save: Option<String>,
    #[serde(rename = "save_per_month", skip_serializing_if = "Option::is_none")]
    pub save_per_month: Option<String>,
    /// The date you started with this piggy bank.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The date you intend to finish saving money.
    #[serde(rename = "target_date", skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The group ID of the group this object is part of. NULL if no group.
    #[serde(rename = "object_group_id", skip_serializing_if = "Option::is_none")]
    pub object_group_id: Option<String>,
    /// The order of the group. At least 1, for the highest sorting.
    #[serde(rename = "object_group_order", skip_serializing_if = "Option::is_none")]
    pub object_group_order: Option<i32>,
    /// The name of the group. NULL if no group.
    #[serde(rename = "object_group_title", skip_serializing_if = "Option::is_none")]
    pub object_group_title: Option<String>,
}

impl PiggyBank {
    pub fn new(account_id: String, name: String, target_amount: String) -> PiggyBank {
        PiggyBank {
            created_at: None,
            updated_at: None,
            account_id,
            account_name: None,
            name,
            currency_id: None,
            currency_code: None,
            currency_symbol: None,
            currency_decimal_places: None,
            target_amount,
            percentage: None,
            current_amount: None,
            left_to_save: None,
            save_per_month: None,
            start_date: None,
            target_date: None,
            order: None,
            active: None,
            notes: None,
            object_group_id: None,
            object_group_order: None,
            object_group_title: None,
        }
    }
}


