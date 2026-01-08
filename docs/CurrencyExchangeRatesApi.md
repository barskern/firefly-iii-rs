# \CurrencyExchangeRatesApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_specific_currency_exchange_rate**](CurrencyExchangeRatesApi.md#delete_specific_currency_exchange_rate) | **DELETE** /v1/exchange-rates/{id} | Delete a specific currency exchange rate.
[**delete_specific_currency_exchange_rate_on_date**](CurrencyExchangeRatesApi.md#delete_specific_currency_exchange_rate_on_date) | **DELETE** /v1/exchange-rates/{from}/{to}/{date} | Delete the currency exchange rate from 'from' to 'to' on the specified date.
[**delete_specific_currency_exchange_rates**](CurrencyExchangeRatesApi.md#delete_specific_currency_exchange_rates) | **DELETE** /v1/exchange-rates/{from}/{to} | Deletes ALL currency exchange rates from 'from' to 'to'.
[**list_currency_exchange_rates**](CurrencyExchangeRatesApi.md#list_currency_exchange_rates) | **GET** /v1/exchange-rates | List all exchange rates that Firefly III knows.
[**list_specific_currency_exchange_rate**](CurrencyExchangeRatesApi.md#list_specific_currency_exchange_rate) | **GET** /v1/exchange-rates/{id} | List a single specific exchange rate.
[**list_specific_currency_exchange_rate_on_date**](CurrencyExchangeRatesApi.md#list_specific_currency_exchange_rate_on_date) | **GET** /v1/exchange-rates/{from}/{to}/{date} | List the exchange rate for the from and to-currency on the requested date.
[**list_specific_currency_exchange_rates**](CurrencyExchangeRatesApi.md#list_specific_currency_exchange_rates) | **GET** /v1/exchange-rates/{from}/{to} | List all exchange rates from/to the mentioned currencies.
[**store_currency_exchange_rate**](CurrencyExchangeRatesApi.md#store_currency_exchange_rate) | **POST** /v1/exchange-rates | Store a new currency exchange rate.
[**store_currency_exchange_rates_by_date**](CurrencyExchangeRatesApi.md#store_currency_exchange_rates_by_date) | **POST** /v1/exchange-rates/by-date/{date} | Store new currency exchange rates under this date
[**store_currency_exchange_rates_by_pair**](CurrencyExchangeRatesApi.md#store_currency_exchange_rates_by_pair) | **POST** /v1/exchange-rates/by-currencies/{from}/{to} | Store new currency exchange rates under this from/to pair.
[**update_currency_exchange_rate**](CurrencyExchangeRatesApi.md#update_currency_exchange_rate) | **PUT** /v1/exchange-rates/{id} | Update existing currency exchange rate.
[**update_currency_exchange_rate_by_date**](CurrencyExchangeRatesApi.md#update_currency_exchange_rate_by_date) | **PUT** /v1/exchange-rates/{from}/{to}/{date} | Update existing currency exchange rate.



## delete_specific_currency_exchange_rate

> delete_specific_currency_exchange_rate(id, x_trace_id)
Delete a specific currency exchange rate.

Delete a specific currency exchange rate by its internal ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the requested currency exchange rate. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_specific_currency_exchange_rate_on_date

> delete_specific_currency_exchange_rate_on_date(from, to, date, x_trace_id)
Delete the currency exchange rate from 'from' to 'to' on the specified date.

Delete the currency exchange rate from 'from' to 'to' on the specified date.  It's important to know that the reverse exchange rate (from 'to' to 'from') will not be deleted and Firefly III will still be able to infer the correct exchange rate from the reverse one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | The currency code of the 'from' currency. | [required] |
**to** | **String** | The currency code of the 'to' currency. | [required] |
**date** | **String** |  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_specific_currency_exchange_rates

> delete_specific_currency_exchange_rates(from, to, x_trace_id)
Deletes ALL currency exchange rates from 'from' to 'to'.

Deletes ALL currency exchange rates from 'from' to 'to'. It's important to know that the reverse exchange rates (from 'to' to 'from') will not be deleted and Firefly III will still be able to infer the correct exchange rate from the reverse one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | The currency code of the 'from' currency. | [required] |
**to** | **String** | The currency code of the 'to' currency. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_currency_exchange_rates

> models::CurrencyExchangeRateArray list_currency_exchange_rates(x_trace_id, limit, page)
List all exchange rates that Firefly III knows.

List exchange rates that Firefly III knows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::CurrencyExchangeRateArray**](CurrencyExchangeRateArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_specific_currency_exchange_rate

> models::CurrencyExchangeRateSingle list_specific_currency_exchange_rate(id, x_trace_id, limit, page)
List a single specific exchange rate.

List a single specific exchange rate by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the requested currency exchange rate. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::CurrencyExchangeRateSingle**](CurrencyExchangeRateSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_specific_currency_exchange_rate_on_date

> models::CurrencyExchangeRateArray list_specific_currency_exchange_rate_on_date(from, to, date, x_trace_id, limit, page)
List the exchange rate for the from and to-currency on the requested date.

List the exchange rate for the from and to-currency on the requested date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | The currency code of the 'from' currency. | [required] |
**to** | **String** | The currency code of the 'to' currency. | [required] |
**date** | **String** |  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::CurrencyExchangeRateArray**](CurrencyExchangeRateArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_specific_currency_exchange_rates

> models::CurrencyExchangeRateArray list_specific_currency_exchange_rates(from, to, x_trace_id, limit, page)
List all exchange rates from/to the mentioned currencies.

List all exchange rates from/to the mentioned currencies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | The currency code of the 'from' currency. | [required] |
**to** | **String** | The currency code of the 'to' currency. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::CurrencyExchangeRateArray**](CurrencyExchangeRateArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_currency_exchange_rate

> models::CurrencyExchangeRateSingle store_currency_exchange_rate(currency_exchange_rate_store, x_trace_id)
Store a new currency exchange rate.

Stores a new exchange rate. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_exchange_rate_store** | [**CurrencyExchangeRateStore**](CurrencyExchangeRateStore.md) | JSON array or key=value pairs with the necessary exchange rate information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::CurrencyExchangeRateSingle**](CurrencyExchangeRateSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_currency_exchange_rates_by_date

> models::CurrencyExchangeRateArray store_currency_exchange_rates_by_date(date, currency_exchange_rate_store_by_date, x_trace_id)
Store new currency exchange rates under this date

Stores a new set of exchange rates. The date is fixed (in the URL parameter) and the data required can be submitted as a JSON body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** |  | [required] |
**currency_exchange_rate_store_by_date** | [**CurrencyExchangeRateStoreByDate**](CurrencyExchangeRateStoreByDate.md) | JSON array with the necessary currency exchange rate information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::CurrencyExchangeRateArray**](CurrencyExchangeRateArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_currency_exchange_rates_by_pair

> models::CurrencyExchangeRateArray store_currency_exchange_rates_by_pair(from, to, request_body, x_trace_id)
Store new currency exchange rates under this from/to pair.

Stores a new set of exchange rates for this pair. The date is variable, and the data required can be submitted as a JSON body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | The currency code of the 'from' currency. | [required] |
**to** | **String** | The currency code of the 'to' currency. | [required] |
**request_body** | [**std::collections::HashMap<String, String>**](String.md) | JSON array with the necessary currency exchange rate information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::CurrencyExchangeRateArray**](CurrencyExchangeRateArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_currency_exchange_rate

> models::CurrencyExchangeRateSingle update_currency_exchange_rate(id, currency_exchange_rate_update, x_trace_id)
Update existing currency exchange rate.

Used to update a single currency exchange rate by its ID. Including the from/to currency is optional. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the currency exchange rate. | [required] |
**currency_exchange_rate_update** | [**CurrencyExchangeRateUpdate**](CurrencyExchangeRateUpdate.md) | JSON array or form-data with updated exchange rate information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::CurrencyExchangeRateSingle**](CurrencyExchangeRateSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_currency_exchange_rate_by_date

> models::CurrencyExchangeRateSingle update_currency_exchange_rate_by_date(from, to, date, currency_exchange_rate_update_no_date, x_trace_id)
Update existing currency exchange rate.

Used to update a single currency exchange rate by its currency codes and date 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | The currency code of the 'from' currency. | [required] |
**to** | **String** | The currency code of the 'to' currency. | [required] |
**date** | **String** |  | [required] |
**currency_exchange_rate_update_no_date** | [**CurrencyExchangeRateUpdateNoDate**](CurrencyExchangeRateUpdateNoDate.md) | JSON array or form-data with updated exchange rate information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::CurrencyExchangeRateSingle**](CurrencyExchangeRateSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

