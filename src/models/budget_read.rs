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
pub struct BudgetRead {
    /// Immutable value
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "attributes")]
    pub attributes: crate::models::Budget,
}

impl BudgetRead {
    pub fn new(_type: String, id: String, attributes: crate::models::Budget) -> BudgetRead {
        BudgetRead {
            _type,
            id,
            attributes: attributes,
        }
    }
}


