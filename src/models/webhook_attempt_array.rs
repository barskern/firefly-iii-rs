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
pub struct WebhookAttemptArray {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::WebhookAttemptRead>,
    #[serde(rename = "meta")]
    pub meta: crate::models::Meta,
}

impl WebhookAttemptArray {
    pub fn new(data: Vec<crate::models::WebhookAttemptRead>, meta: crate::models::Meta) -> WebhookAttemptArray {
        WebhookAttemptArray {
            data,
            meta: meta,
        }
    }
}


