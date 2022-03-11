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
pub struct PreferenceSingle {
    #[serde(rename = "data")]
    pub data: crate::models::PreferenceRead,
}

impl PreferenceSingle {
    pub fn new(data: crate::models::PreferenceRead) -> PreferenceSingle {
        PreferenceSingle {
            data: data,
        }
    }
}


