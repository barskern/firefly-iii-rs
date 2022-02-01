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
pub struct CronResultRow {
    /// This value tells you if this specific cron job actually fired. It may not fire. Some cron jobs only fire every 24 hours, for example. 
    #[serde(rename = "job_fired", skip_serializing_if = "Option::is_none")]
    pub job_fired: Option<bool>,
    /// This value tells you if this specific cron job actually did something. The job may fire but not change anything. 
    #[serde(rename = "job_succeeded", skip_serializing_if = "Option::is_none")]
    pub job_succeeded: Option<bool>,
    /// If the cron job ran into some kind of an error, this value will be true.
    #[serde(rename = "job_errored", skip_serializing_if = "Option::is_none")]
    pub job_errored: Option<bool>,
    /// If the cron job ran into some kind of an error, this value will be the error message. The success message if the job actually ran OK. 
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl CronResultRow {
    pub fn new() -> CronResultRow {
        CronResultRow {
            job_fired: None,
            job_succeeded: None,
            job_errored: None,
            message: None,
        }
    }
}


