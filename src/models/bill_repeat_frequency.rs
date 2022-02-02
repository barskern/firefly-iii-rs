/*
 * Firefly III API v1.5.5
 *
 * This is the documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. You may use the \"Authorize\" button to try the API below. This file was last generated on 2022-01-30T05:47:28+00:00 
 *
 * The version of the OpenAPI document: 1.5.5
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */

/// BillRepeatFrequency : How often the bill must be paid.

/// How often the bill must be paid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillRepeatFrequency {
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "quarterly")]
    Quarterly,
    #[serde(rename = "half-year")]
    HalfYear,
    #[serde(rename = "yearly")]
    Yearly,

}

impl ToString for BillRepeatFrequency {
    fn to_string(&self) -> String {
        match self {
            Self::Weekly => String::from("weekly"),
            Self::Monthly => String::from("monthly"),
            Self::Quarterly => String::from("quarterly"),
            Self::HalfYear => String::from("half-year"),
            Self::Yearly => String::from("yearly"),
        }
    }
}

impl std::default::Default for BillRepeatFrequency {
    fn default() -> BillRepeatFrequency {
        Self::Weekly
    }
}



