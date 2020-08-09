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

pub struct CurrencyExchangeRatesApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl CurrencyExchangeRatesApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> CurrencyExchangeRatesApiClient {
        CurrencyExchangeRatesApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait CurrencyExchangeRatesApi {
    async fn get_exchange_rate(&self, from: Option<&str>, to: Option<&str>, date: Option<String>, amount: Option<f64>) -> Result<crate::models::ExchangeRate, Error>;
}

#[async_trait]
impl CurrencyExchangeRatesApi for CurrencyExchangeRatesApiClient {
    async fn get_exchange_rate(&self, from: Option<&str>, to: Option<&str>, date: Option<String>, amount: Option<f64>) -> Result<crate::models::ExchangeRate, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/cer", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = from {
            req_builder = req_builder.query(&[("from", &s.to_string())]);
        }
        if let Some(ref s) = to {
            req_builder = req_builder.query(&[("to", &s.to_string())]);
        }
        if let Some(ref s) = date {
            req_builder = req_builder.query(&[("date", &s.to_string())]);
        }
        if let Some(ref s) = amount {
            req_builder = req_builder.query(&[("amount", &s.to_string())]);
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
