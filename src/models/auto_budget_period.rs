/*
 * Firefly III API v1.5.5
 *
 * This is the documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. You may use the \"Authorize\" button to try the API below. This file was last generated on 2022-01-30T05:47:28+00:00 
 *
 * The version of the OpenAPI document: 1.5.5
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */

/// AutoBudgetPeriod : Period for the auto budget

/// Period for the auto budget
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AutoBudgetPeriod {
    #[serde(rename = "daily")]
    Daily,
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
    #[serde(rename = "null")]
    Null,

}

impl ToString for AutoBudgetPeriod {
    fn to_string(&self) -> String {
        match self {
            Self::Daily => String::from("daily"),
            Self::Weekly => String::from("weekly"),
            Self::Monthly => String::from("monthly"),
            Self::Quarterly => String::from("quarterly"),
            Self::HalfYear => String::from("half-year"),
            Self::Yearly => String::from("yearly"),
            Self::Null => String::from("null"),
        }
    }
}

impl std::default::Default for AutoBudgetPeriod {
    fn default() -> AutoBudgetPeriod {
        Self::Daily
    }
}



