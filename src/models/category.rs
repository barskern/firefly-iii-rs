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
pub struct Category {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "spent", skip_serializing_if = "Option::is_none")]
    pub spent: Option<Vec<crate::models::CategorySpent>>,
    #[serde(rename = "earned", skip_serializing_if = "Option::is_none")]
    pub earned: Option<Vec<crate::models::CategoryEarned>>,
}

impl Category {
    pub fn new(name: String) -> Category {
        Category {
            created_at: None,
            updated_at: None,
            name,
            spent: None,
            earned: None,
        }
    }
}


