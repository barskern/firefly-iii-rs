# \SearchApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_accounts**](SearchApi.md#search_accounts) | **GET** /api/v1/search/accounts | Search for accounts
[**search_transactions**](SearchApi.md#search_transactions) | **GET** /api/v1/search/transactions | Search for transactions



## search_accounts

> crate::models::AccountArray search_accounts(query, field, page, _type)
Search for accounts

Search for accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | The query you wish to search for. | [required] |
**field** | [**crate::models::AccountSearchFieldFilter**](.md) | The account field(s) you want to search in. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50 |  |
**_type** | Option<[**crate::models::AccountTypeFilter**](.md)> | The type of accounts you wish to limit the search to. |  |

### Return type

[**crate::models::AccountArray**](AccountArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_transactions

> crate::models::TransactionArray search_transactions(query, page)
Search for transactions

Searches through the users transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | The query you wish to search for. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50 |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

