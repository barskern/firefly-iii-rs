/*
 * Firefly III API v1.5.5
 *
 * This is the documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. You may use the \"Authorize\" button to try the API below. This file was last generated on 2022-01-30T05:47:28+00:00 
 *
 * The version of the OpenAPI document: 1.5.5
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionTypeProperty {
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "reconciliation")]
    Reconciliation,
    #[serde(rename = "opening balance")]
    OpeningBalance,

}

impl ToString for TransactionTypeProperty {
    fn to_string(&self) -> String {
        match self {
            Self::Withdrawal => String::from("withdrawal"),
            Self::Deposit => String::from("deposit"),
            Self::Transfer => String::from("transfer"),
            Self::Reconciliation => String::from("reconciliation"),
            Self::OpeningBalance => String::from("opening balance"),
        }
    }
}

impl std::default::Default for TransactionTypeProperty {
    fn default() -> TransactionTypeProperty {
        Self::Withdrawal
    }
}




