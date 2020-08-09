/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.3.0
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

pub struct SearchApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl SearchApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> SearchApiClient {
        SearchApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait SearchApi {
    async fn search_accounts(&self, query: &str, _type: crate::models::AccountTypeFilter, field: crate::models::AccountSearchFieldFilter, page: Option<i32>) -> Result<crate::models::AccountArray, Error>;
    async fn search_transactions(&self, query: &str, page: Option<i32>) -> Result<crate::models::TransactionArray, Error>;
}

#[async_trait]
impl SearchApi for SearchApiClient {
    async fn search_accounts(&self, query: &str, _type: crate::models::AccountTypeFilter, field: crate::models::AccountSearchFieldFilter, page: Option<i32>) -> Result<crate::models::AccountArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/search/accounts", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("query", &query.to_string())]);
        req_builder = req_builder.query(&[("type", &_type.to_string())]);
        req_builder = req_builder.query(&[("field", &field.to_string())]);
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

    async fn search_transactions(&self, query: &str, page: Option<i32>) -> Result<crate::models::TransactionArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/search/transactions", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("query", &query.to_string())]);
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

}
