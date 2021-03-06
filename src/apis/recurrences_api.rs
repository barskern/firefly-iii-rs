/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */

use async_trait::async_trait;
use std::sync::Arc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct RecurrencesApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl RecurrencesApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> RecurrencesApiClient {
        RecurrencesApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait RecurrencesApi {
    async fn delete_recurrence(&self, id: i32) -> Result<(), Error>;
    async fn get_recurrence(&self, id: i32) -> Result<crate::models::RecurrenceSingle, Error>;
    async fn list_recurrence(&self, page: Option<i32>) -> Result<crate::models::RecurrenceArray, Error>;
    async fn list_transaction_by_recurrence(&self, id: i32, page: Option<i32>, start: Option<String>, end: Option<String>, _type: Option<crate::models::TransactionTypeFilter>) -> Result<crate::models::TransactionArray, Error>;
    async fn store_recurrence(&self, recurrence: crate::models::Recurrence) -> Result<crate::models::RecurrenceSingle, Error>;
    async fn trigger_recurrence(&self, ) -> Result<(), Error>;
    async fn update_recurrence(&self, id: i32, recurrence: crate::models::Recurrence) -> Result<crate::models::RecurrenceSingle, Error>;
}

#[async_trait]
impl RecurrencesApi for RecurrencesApiClient {
    async fn delete_recurrence(&self, id: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/recurrences/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::DELETE, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    async fn get_recurrence(&self, id: i32) -> Result<crate::models::RecurrenceSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/recurrences/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // ensure returntype is json (only supported returntype)
        req_builder = req_builder.header(reqwest::header::ACCEPT, "application/json");

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn list_recurrence(&self, page: Option<i32>) -> Result<crate::models::RecurrenceArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/recurrences", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // ensure returntype is json (only supported returntype)
        req_builder = req_builder.header(reqwest::header::ACCEPT, "application/json");

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn list_transaction_by_recurrence(&self, id: i32, page: Option<i32>, start: Option<String>, end: Option<String>, _type: Option<crate::models::TransactionTypeFilter>) -> Result<crate::models::TransactionArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/recurrences/{id}/transactions", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = start {
            req_builder = req_builder.query(&[("start", &s.to_string())]);
        }
        if let Some(ref s) = end {
            req_builder = req_builder.query(&[("end", &s.to_string())]);
        }
        if let Some(ref s) = _type {
            req_builder = req_builder.query(&[("type", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // ensure returntype is json (only supported returntype)
        req_builder = req_builder.header(reqwest::header::ACCEPT, "application/json");

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn store_recurrence(&self, recurrence: crate::models::Recurrence) -> Result<crate::models::RecurrenceSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/recurrences", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::POST, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&recurrence);

        // ensure returntype is json (only supported returntype)
        req_builder = req_builder.header(reqwest::header::ACCEPT, "application/json");

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn trigger_recurrence(&self, ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/recurrences/trigger", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::POST, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    async fn update_recurrence(&self, id: i32, recurrence: crate::models::Recurrence) -> Result<crate::models::RecurrenceSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/recurrences/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::PUT, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&recurrence);

        // ensure returntype is json (only supported returntype)
        req_builder = req_builder.header(reqwest::header::ACCEPT, "application/json");

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

}
