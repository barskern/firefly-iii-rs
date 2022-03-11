/*
 * Firefly III API v1.5.5
 *
 * This is the documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. You may use the \"Authorize\" button to try the API below. This file was last generated on 2022-01-30T05:47:28+00:00 
 *
 * The version of the OpenAPI document: 1.5.5
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */

/// AccountRoleProperty : Is only mandatory when the type is asset.

/// Is only mandatory when the type is asset.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountRoleProperty {
    #[serde(rename = "defaultAsset")]
    DefaultAsset,
    #[serde(rename = "sharedAsset")]
    SharedAsset,
    #[serde(rename = "savingAsset")]
    SavingAsset,
    #[serde(rename = "ccAsset")]
    CcAsset,
    #[serde(rename = "cashWalletAsset")]
    CashWalletAsset,
    #[serde(rename = "null")]
    Null,

}

impl ToString for AccountRoleProperty {
    fn to_string(&self) -> String {
        match self {
            Self::DefaultAsset => String::from("defaultAsset"),
            Self::SharedAsset => String::from("sharedAsset"),
            Self::SavingAsset => String::from("savingAsset"),
            Self::CcAsset => String::from("ccAsset"),
            Self::CashWalletAsset => String::from("cashWalletAsset"),
            Self::Null => String::from("null"),
        }
    }
}

impl std::default::Default for AccountRoleProperty {
    fn default() -> AccountRoleProperty {
        Self::DefaultAsset
    }
}




