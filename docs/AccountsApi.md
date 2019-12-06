# \AccountsApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_account**](AccountsApi.md#delete_account) | **DELETE** /api/v1/accounts/{id} | Permanently delete account.
[**get_account**](AccountsApi.md#get_account) | **GET** /api/v1/accounts/{id} | Get single account.
[**list_account**](AccountsApi.md#list_account) | **GET** /api/v1/accounts | List all accounts.
[**list_piggy_bank_by_account**](AccountsApi.md#list_piggy_bank_by_account) | **GET** /api/v1/accounts/{id}/piggy_banks | List all piggy banks related to the account.
[**list_transaction_by_account**](AccountsApi.md#list_transaction_by_account) | **GET** /api/v1/accounts/{id}/transactions | List all transactions related to the account.
[**store_account**](AccountsApi.md#store_account) | **POST** /api/v1/accounts | Create new account.
[**update_account**](AccountsApi.md#update_account) | **PUT** /api/v1/accounts/{id} | Update existing account.



## delete_account

> delete_account(id)
Permanently delete account.

Will permanently delete an account. Any associated transactions and piggy banks are ALSO deleted. Cannot be recovered from. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the account. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account

> crate::models::AccountSingle get_account(id, date)
Get single account.

Returns a single account by its ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the account. | [required] |
**date** | Option<**String**> | A date formatted YYYY-MM-DD. When added to the request, Firefly III will show the account's balance on that day.  |  |

### Return type

[**crate::models::AccountSingle**](AccountSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_account

> crate::models::AccountArray list_account(page, date, _type)
List all accounts.

This endpoint returns a list of all the accounts owned by the authenticated user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**date** | Option<**String**> | A date formatted YYYY-MM-DD. When added to the request, Firefly III will show the account's balance on that day.  |  |
**_type** | Option<**String**> | Optional filter on the account type(s) returned |  |

### Return type

[**crate::models::AccountArray**](AccountArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_piggy_bank_by_account

> crate::models::PiggyBankArray list_piggy_bank_by_account(id, page)
List all piggy banks related to the account.

This endpoint returns a list of all the piggy banks connected to the account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the account. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**crate::models::PiggyBankArray**](PiggyBankArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_account

> crate::models::TransactionArray list_transaction_by_account(id, page, limit, start, end, _type)
List all transactions related to the account.

This endpoint returns a list of all the transactions connected to the account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the account. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**limit** | Option<**i32**> | Limits the number of results on one page. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**_type** | Option<**String**> | Optional filter on the transaction type(s) returned. |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_account

> crate::models::AccountSingle store_account(account)
Create new account.

Creates a new account. The data required can be submitted as a JSON body or as a list of parameters (in key=value pairs, like a webform).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account** | [**Account**](Account.md) | JSON array with the necessary account information or key=value pairs. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::AccountSingle**](AccountSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_account

> crate::models::AccountSingle update_account(id, account)
Update existing account.

Used to update a single account. All fields that are not submitted will be cleared (set to NULL). The model will tell you which fields are mandatory. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the account. | [required] |
**account** | [**Account**](Account.md) | JSON array or formdata with updated account information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::AccountSingle**](AccountSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

