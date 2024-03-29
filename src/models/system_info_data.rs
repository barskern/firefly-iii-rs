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
pub struct SystemInfoData {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "api_version")]
    pub api_version: String,
    #[serde(rename = "php_version")]
    pub php_version: String,
    #[serde(rename = "os")]
    pub os: String,
    #[serde(rename = "driver")]
    pub driver: String,
}

impl SystemInfoData {
    pub fn new(version: String, api_version: String, php_version: String, os: String, driver: String) -> SystemInfoData {
        SystemInfoData {
            version,
            api_version,
            php_version,
            os,
            driver,
        }
    }
}


