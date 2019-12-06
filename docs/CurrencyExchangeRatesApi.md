# \CurrencyExchangeRatesApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_exchange_rate**](CurrencyExchangeRatesApi.md#get_exchange_rate) | **GET** /api/v1/cer | Get an exchange rate.



## get_exchange_rate

> crate::models::ExchangeRate get_exchange_rate(from, to, date, amount)
Get an exchange rate.

Get an exchange rate. If Firefly III doesn't know the rate it will set the rate to 1.0.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> | The source currency code. If omitted, defaults to EUR. |  |
**to** | Option<**String**> | The destination currency code. If omitted, defaults to USD. |  |
**date** | Option<**String**> | The date you want to know the exchange rate on. |  |
**amount** | Option<**f64**> | The amount in the source currency. If added, Firefly III will calculate the amount in the destination currency. |  |

### Return type

[**crate::models::ExchangeRate**](ExchangeRate.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

