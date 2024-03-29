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
pub struct AutocompleteRecurrence {
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the recurrence found by an auto-complete search.
    #[serde(rename = "name")]
    pub name: String,
    /// Description of the recurrence found by auto-complete.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AutocompleteRecurrence {
    pub fn new(id: String, name: String) -> AutocompleteRecurrence {
        AutocompleteRecurrence {
            id,
            name,
            description: None,
        }
    }
}


