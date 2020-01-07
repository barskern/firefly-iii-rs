use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(_) => write!(f, "reqwest error"),
            Self::Serde(_) => write!(f, "serde error"),
            Self::Io(_) => write!(f, "i/o error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Reqwest(e) => Some(e),
            Self::Serde(e) => Some(e),
            Self::Io(e) => Some(e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod about_api;
pub use self::about_api::{ AboutApi, AboutApiClient };
mod accounts_api;
pub use self::accounts_api::{ AccountsApi, AccountsApiClient };
mod attachments_api;
pub use self::attachments_api::{ AttachmentsApi, AttachmentsApiClient };
mod available_budgets_api;
pub use self::available_budgets_api::{ AvailableBudgetsApi, AvailableBudgetsApiClient };
mod bills_api;
pub use self::bills_api::{ BillsApi, BillsApiClient };
mod budgets_api;
pub use self::budgets_api::{ BudgetsApi, BudgetsApiClient };
mod categories_api;
pub use self::categories_api::{ CategoriesApi, CategoriesApiClient };
mod charts_api;
pub use self::charts_api::{ ChartsApi, ChartsApiClient };
mod configuration_api;
pub use self::configuration_api::{ ConfigurationApi, ConfigurationApiClient };
mod currencies_api;
pub use self::currencies_api::{ CurrenciesApi, CurrenciesApiClient };
mod currency_exchange_rates_api;
pub use self::currency_exchange_rates_api::{ CurrencyExchangeRatesApi, CurrencyExchangeRatesApiClient };
mod import_api;
pub use self::import_api::{ ImportApi, ImportApiClient };
mod links_api;
pub use self::links_api::{ LinksApi, LinksApiClient };
mod piggy_banks_api;
pub use self::piggy_banks_api::{ PiggyBanksApi, PiggyBanksApiClient };
mod preferences_api;
pub use self::preferences_api::{ PreferencesApi, PreferencesApiClient };
mod recurrences_api;
pub use self::recurrences_api::{ RecurrencesApi, RecurrencesApiClient };
mod rule_groups_api;
pub use self::rule_groups_api::{ RuleGroupsApi, RuleGroupsApiClient };
mod rules_api;
pub use self::rules_api::{ RulesApi, RulesApiClient };
mod summary_api;
pub use self::summary_api::{ SummaryApi, SummaryApiClient };
mod tags_api;
pub use self::tags_api::{ TagsApi, TagsApiClient };
mod transactions_api;
pub use self::transactions_api::{ TransactionsApi, TransactionsApiClient };
mod users_api;
pub use self::users_api::{ UsersApi, UsersApiClient };

pub mod configuration;
pub mod client;
