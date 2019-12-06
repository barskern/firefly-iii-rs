use std::sync::Arc;

use super::configuration::Configuration;

pub struct APIClient {
    about_api: Box<dyn crate::apis::AboutApi>,
    accounts_api: Box<dyn crate::apis::AccountsApi>,
    attachments_api: Box<dyn crate::apis::AttachmentsApi>,
    available_budgets_api: Box<dyn crate::apis::AvailableBudgetsApi>,
    bills_api: Box<dyn crate::apis::BillsApi>,
    budgets_api: Box<dyn crate::apis::BudgetsApi>,
    categories_api: Box<dyn crate::apis::CategoriesApi>,
    charts_api: Box<dyn crate::apis::ChartsApi>,
    configuration_api: Box<dyn crate::apis::ConfigurationApi>,
    currencies_api: Box<dyn crate::apis::CurrenciesApi>,
    currency_exchange_rates_api: Box<dyn crate::apis::CurrencyExchangeRatesApi>,
    import_api: Box<dyn crate::apis::ImportApi>,
    links_api: Box<dyn crate::apis::LinksApi>,
    piggy_banks_api: Box<dyn crate::apis::PiggyBanksApi>,
    preferences_api: Box<dyn crate::apis::PreferencesApi>,
    recurrences_api: Box<dyn crate::apis::RecurrencesApi>,
    rule_groups_api: Box<dyn crate::apis::RuleGroupsApi>,
    rules_api: Box<dyn crate::apis::RulesApi>,
    summary_api: Box<dyn crate::apis::SummaryApi>,
    tags_api: Box<dyn crate::apis::TagsApi>,
    transactions_api: Box<dyn crate::apis::TransactionsApi>,
    users_api: Box<dyn crate::apis::UsersApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let arc = Arc::new(configuration);

        APIClient {
            about_api: Box::new(crate::apis::AboutApiClient::new(arc.clone())),
            accounts_api: Box::new(crate::apis::AccountsApiClient::new(arc.clone())),
            attachments_api: Box::new(crate::apis::AttachmentsApiClient::new(arc.clone())),
            available_budgets_api: Box::new(crate::apis::AvailableBudgetsApiClient::new(arc.clone())),
            bills_api: Box::new(crate::apis::BillsApiClient::new(arc.clone())),
            budgets_api: Box::new(crate::apis::BudgetsApiClient::new(arc.clone())),
            categories_api: Box::new(crate::apis::CategoriesApiClient::new(arc.clone())),
            charts_api: Box::new(crate::apis::ChartsApiClient::new(arc.clone())),
            configuration_api: Box::new(crate::apis::ConfigurationApiClient::new(arc.clone())),
            currencies_api: Box::new(crate::apis::CurrenciesApiClient::new(arc.clone())),
            currency_exchange_rates_api: Box::new(crate::apis::CurrencyExchangeRatesApiClient::new(arc.clone())),
            import_api: Box::new(crate::apis::ImportApiClient::new(arc.clone())),
            links_api: Box::new(crate::apis::LinksApiClient::new(arc.clone())),
            piggy_banks_api: Box::new(crate::apis::PiggyBanksApiClient::new(arc.clone())),
            preferences_api: Box::new(crate::apis::PreferencesApiClient::new(arc.clone())),
            recurrences_api: Box::new(crate::apis::RecurrencesApiClient::new(arc.clone())),
            rule_groups_api: Box::new(crate::apis::RuleGroupsApiClient::new(arc.clone())),
            rules_api: Box::new(crate::apis::RulesApiClient::new(arc.clone())),
            summary_api: Box::new(crate::apis::SummaryApiClient::new(arc.clone())),
            tags_api: Box::new(crate::apis::TagsApiClient::new(arc.clone())),
            transactions_api: Box::new(crate::apis::TransactionsApiClient::new(arc.clone())),
            users_api: Box::new(crate::apis::UsersApiClient::new(arc.clone())),
        }
    }

    pub fn about_api(&self) -> &dyn crate::apis::AboutApi{
        self.about_api.as_ref()
    }

    pub fn accounts_api(&self) -> &dyn crate::apis::AccountsApi{
        self.accounts_api.as_ref()
    }

    pub fn attachments_api(&self) -> &dyn crate::apis::AttachmentsApi{
        self.attachments_api.as_ref()
    }

    pub fn available_budgets_api(&self) -> &dyn crate::apis::AvailableBudgetsApi{
        self.available_budgets_api.as_ref()
    }

    pub fn bills_api(&self) -> &dyn crate::apis::BillsApi{
        self.bills_api.as_ref()
    }

    pub fn budgets_api(&self) -> &dyn crate::apis::BudgetsApi{
        self.budgets_api.as_ref()
    }

    pub fn categories_api(&self) -> &dyn crate::apis::CategoriesApi{
        self.categories_api.as_ref()
    }

    pub fn charts_api(&self) -> &dyn crate::apis::ChartsApi{
        self.charts_api.as_ref()
    }

    pub fn configuration_api(&self) -> &dyn crate::apis::ConfigurationApi{
        self.configuration_api.as_ref()
    }

    pub fn currencies_api(&self) -> &dyn crate::apis::CurrenciesApi{
        self.currencies_api.as_ref()
    }

    pub fn currency_exchange_rates_api(&self) -> &dyn crate::apis::CurrencyExchangeRatesApi{
        self.currency_exchange_rates_api.as_ref()
    }

    pub fn import_api(&self) -> &dyn crate::apis::ImportApi{
        self.import_api.as_ref()
    }

    pub fn links_api(&self) -> &dyn crate::apis::LinksApi{
        self.links_api.as_ref()
    }

    pub fn piggy_banks_api(&self) -> &dyn crate::apis::PiggyBanksApi{
        self.piggy_banks_api.as_ref()
    }

    pub fn preferences_api(&self) -> &dyn crate::apis::PreferencesApi{
        self.preferences_api.as_ref()
    }

    pub fn recurrences_api(&self) -> &dyn crate::apis::RecurrencesApi{
        self.recurrences_api.as_ref()
    }

    pub fn rule_groups_api(&self) -> &dyn crate::apis::RuleGroupsApi{
        self.rule_groups_api.as_ref()
    }

    pub fn rules_api(&self) -> &dyn crate::apis::RulesApi{
        self.rules_api.as_ref()
    }

    pub fn summary_api(&self) -> &dyn crate::apis::SummaryApi{
        self.summary_api.as_ref()
    }

    pub fn tags_api(&self) -> &dyn crate::apis::TagsApi{
        self.tags_api.as_ref()
    }

    pub fn transactions_api(&self) -> &dyn crate::apis::TransactionsApi{
        self.transactions_api.as_ref()
    }

    pub fn users_api(&self) -> &dyn crate::apis::UsersApi{
        self.users_api.as_ref()
    }

}
