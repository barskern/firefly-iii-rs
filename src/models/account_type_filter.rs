/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 0.10.4
 * Contact: thegrumpydictator@gmail.com
 * Generated by: https://openapi-generator.tech
 */


use serde::{Deserialize, Serialize};

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountTypeFilter {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "asset")]
    Asset,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "expense")]
    Expense,
    #[serde(rename = "revenue")]
    Revenue,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "liability")]
    Liability,
    #[serde(rename = "liabilities")]
    Liabilities,
    #[serde(rename = "Default account")]
    DefaultAccount,
    #[serde(rename = "Cash account")]
    CashAccount,
    #[serde(rename = "Asset account")]
    AssetAccount,
    #[serde(rename = "Expense account")]
    ExpenseAccount,
    #[serde(rename = "Revenue account")]
    RevenueAccount,
    #[serde(rename = "Initial balance account")]
    InitialBalanceAccount,
    #[serde(rename = "Beneficiary account")]
    BeneficiaryAccount,
    #[serde(rename = "Import account")]
    ImportAccount,
    #[serde(rename = "Reconciliation account")]
    ReconciliationAccount,
    #[serde(rename = "Loan")]
    Loan,
    #[serde(rename = "Debt")]
    Debt,
    #[serde(rename = "Mortgage")]
    Mortgage,

}

impl std::string::ToString for AccountTypeFilter {
    fn to_string(&self) -> String {
        match self {
        
            Self::All => "all".to_string(),
        
            Self::Asset => "asset".to_string(),
        
            Self::Cash => "cash".to_string(),
        
            Self::Expense => "expense".to_string(),
        
            Self::Revenue => "revenue".to_string(),
        
            Self::Special => "special".to_string(),
        
            Self::Hidden => "hidden".to_string(),
        
            Self::Liability => "liability".to_string(),
        
            Self::Liabilities => "liabilities".to_string(),
        
            Self::DefaultAccount => "Default account".to_string(),
        
            Self::CashAccount => "Cash account".to_string(),
        
            Self::AssetAccount => "Asset account".to_string(),
        
            Self::ExpenseAccount => "Expense account".to_string(),
        
            Self::RevenueAccount => "Revenue account".to_string(),
        
            Self::InitialBalanceAccount => "Initial balance account".to_string(),
        
            Self::BeneficiaryAccount => "Beneficiary account".to_string(),
        
            Self::ImportAccount => "Import account".to_string(),
        
            Self::ReconciliationAccount => "Reconciliation account".to_string(),
        
            Self::Loan => "Loan".to_string(),
        
            Self::Debt => "Debt".to_string(),
        
            Self::Mortgage => "Mortgage".to_string(),
        
        }
    }
}




