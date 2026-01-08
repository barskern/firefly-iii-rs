# \AccountsApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_account**](AccountsApi.md#delete_account) | **DELETE** /v1/accounts/{id} | Permanently delete account.
[**get_account**](AccountsApi.md#get_account) | **GET** /v1/accounts/{id} | Get single account.
[**list_account**](AccountsApi.md#list_account) | **GET** /v1/accounts | List all accounts.
[**list_attachment_by_account**](AccountsApi.md#list_attachment_by_account) | **GET** /v1/accounts/{id}/attachments | Lists all attachments.
[**list_piggy_bank_by_account**](AccountsApi.md#list_piggy_bank_by_account) | **GET** /v1/accounts/{id}/piggy-banks | List all piggy banks related to the account.
[**list_transaction_by_account**](AccountsApi.md#list_transaction_by_account) | **GET** /v1/accounts/{id}/transactions | List all transactions related to the account.
[**store_account**](AccountsApi.md#store_account) | **POST** /v1/accounts | Create new account.
[**update_account**](AccountsApi.md#update_account) | **PUT** /v1/accounts/{id} | Update existing account.



## delete_account

> delete_account(id, x_trace_id)
Permanently delete account.

Will permanently delete an account. Any associated transactions and piggy banks are ALSO deleted. Cannot be recovered from. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account

> models::AccountSingle get_account(id, x_trace_id, start, end, date)
Get single account.

Returns a single account by its ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD. May be omitted.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD. Must be after \"start\". Can not be the same as \"start\". May be omitted.  |  |
**date** | Option<**String**> | A date formatted YYYY-MM-DD. When added to the request, Firefly III will show the account's balance on that day.  |  |

### Return type

[**models::AccountSingle**](AccountSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_account

> models::AccountArray list_account(x_trace_id, limit, page, start, end, date, r#type)
List all accounts.

This endpoint returns a list of all the accounts owned by the authenticated user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD. May be omitted.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD. Must be after \"start\". Can not be the same as \"start\". May be omitted.  |  |
**date** | Option<**String**> | A date formatted YYYY-MM-DD. When added to the request, Firefly III will show the account's balance on that day.  |  |
**r#type** | Option<[**AccountTypeFilter**](.md)> | Optional filter on the account type(s) returned |  |

### Return type

[**models::AccountArray**](AccountArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment_by_account

> models::AttachmentArray list_attachment_by_account(id, x_trace_id, limit, page)
Lists all attachments.

Lists all attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::AttachmentArray**](AttachmentArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_piggy_bank_by_account

> models::PiggyBankArray list_piggy_bank_by_account(id, x_trace_id, limit, page)
List all piggy banks related to the account.

This endpoint returns a list of all the piggy banks connected to the account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::PiggyBankArray**](PiggyBankArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_account

> models::TransactionArray list_transaction_by_account(id, x_trace_id, limit, page, start, end, r#type)
List all transactions related to the account.

This endpoint returns a list of all the transactions connected to the account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**r#type** | Option<[**TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned. |  |

### Return type

[**models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_account

> models::AccountSingle store_account(account_store, x_trace_id)
Create new account.

Creates a new account. The data required can be submitted as a JSON body or as a list of parameters (in key=value pairs, like a webform).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_store** | [**AccountStore**](AccountStore.md) | JSON array with the necessary account information or key=value pairs. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::AccountSingle**](AccountSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_account

> models::AccountSingle update_account(id, account_update, x_trace_id)
Update existing account.

Used to update a single account. All fields that are not submitted will be cleared (set to NULL). The model will tell you which fields are mandatory. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
**account_update** | [**AccountUpdate**](AccountUpdate.md) | JSON array or form-data with updated account information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::AccountSingle**](AccountSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

