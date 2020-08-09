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
pub struct RecurrenceRepetition {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The type of the repetition. ndom means: the n-th weekday of the month, where you can also specify which day of the week.
    #[serde(rename = "type")]
    pub _type: Type,
    /// Information that defined the type of repetition. - For 'daily', this is empty. - For 'weekly', it is day of the week between 1 and 7 (Monday - Sunday). - For 'ndom', it is '1,2' or '4,5' or something else, where the first number is the week in the month, and the second number is the day in the week (between 1 and 7). '2,3' means: the 2nd Wednesday of the month - For 'monthly' it is the day of the month (1 - 31) - For yearly, it is a full date, ie '2018-09-17'. The year you use does not matter. 
    #[serde(rename = "moment")]
    pub moment: String,
    /// How many occurrences to skip. 0 means skip nothing. 1 means every other.
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    /// How to respond when the recurring transaction falls in the weekend. Possible values: 1. Do nothing, just create it 2. Create no transaction. 3. Skip to the previous Friday. 4. Skip to the next Monday. 
    #[serde(rename = "weekend", skip_serializing_if = "Option::is_none")]
    pub weekend: Option<i32>,
    /// Auto-generated repetition description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Array of future dates when the repetition will apply to. Auto generated.
    #[serde(rename = "occurrences", skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<Vec<String>>,
}

impl RecurrenceRepetition {
    pub fn new(_type: Type, moment: String) -> RecurrenceRepetition {
        RecurrenceRepetition {
            id: None,
            created_at: None,
            updated_at: None,
            _type,
            moment,
            skip: None,
            weekend: None,
            description: None,
            occurrences: None,
        }
    }
}

/// The type of the repetition. ndom means: the n-th weekday of the month, where you can also specify which day of the week.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "ndom")]
    Ndom,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "yearly")]
    Yearly,
}

