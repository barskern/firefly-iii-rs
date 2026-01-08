# \SearchApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_accounts**](SearchApi.md#search_accounts) | **GET** /v1/search/accounts | Search for accounts
[**search_transactions**](SearchApi.md#search_transactions) | **GET** /v1/search/transactions | Search for transactions



## search_accounts

> models::AccountArray search_accounts(query, field, x_trace_id, limit, page, r#type)
Search for accounts

Search for accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | The query you wish to search for. | [required] |
**field** | [**AccountSearchFieldFilter**](.md) | The account field(s) you want to search in. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**r#type** | Option<[**AccountTypeFilter**](.md)> | The type of accounts you wish to limit the search to. |  |

### Return type

[**models::AccountArray**](AccountArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_transactions

> models::TransactionArray search_transactions(query, x_trace_id, limit, page)
Search for transactions

Searches through the users transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | The query you wish to search for. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

