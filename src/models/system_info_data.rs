/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */


use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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


