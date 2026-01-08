# \SummaryApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_basic_summary**](SummaryApi.md#get_basic_summary) | **GET** /v1/summary/basic | Returns basic sums of the users data.



## get_basic_summary

> std::collections::HashMap<String, models::BasicSummaryEntry> get_basic_summary(start, end, x_trace_id, currency_code)
Returns basic sums of the users data.

Returns basic sums of the users data, like the net worth, spent and earned amounts. It is multi-currency, and is used in Firefly III to populate the dashboard. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**currency_code** | Option<**String**> | A currency code like EUR or USD, to filter the result.  |  |

### Return type

[**std::collections::HashMap<String, models::BasicSummaryEntry>**](BasicSummaryEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

