# \RecurrencesApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_recurrence**](RecurrencesApi.md#delete_recurrence) | **DELETE** /api/v1/recurrences/{id} | Delete a recurring transaction.
[**get_recurrence**](RecurrencesApi.md#get_recurrence) | **GET** /api/v1/recurrences/{id} | Get a single recurring transaction.
[**list_recurrence**](RecurrencesApi.md#list_recurrence) | **GET** /api/v1/recurrences | List all recurring transactions.
[**list_transaction_by_recurrence**](RecurrencesApi.md#list_transaction_by_recurrence) | **GET** /api/v1/recurrences/{id}/transactions | List all transactions created by a recurring transaction.
[**store_recurrence**](RecurrencesApi.md#store_recurrence) | **POST** /api/v1/recurrences | Store a new recurring transaction
[**update_recurrence**](RecurrencesApi.md#update_recurrence) | **PUT** /api/v1/recurrences/{id} | Update existing recurring transaction.



## delete_recurrence

> delete_recurrence(id)
Delete a recurring transaction.

Delete a recurring transaction. Transactions created by the recurring transaction will not be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the recurring transaction. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recurrence

> crate::models::RecurrenceSingle get_recurrence(id)
Get a single recurring transaction.

Get a single recurring transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the recurring transaction. | [required] |

### Return type

[**crate::models::RecurrenceSingle**](RecurrenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recurrence

> crate::models::RecurrenceArray list_recurrence(page)
List all recurring transactions.

List all recurring transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::RecurrenceArray**](RecurrenceArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_recurrence

> crate::models::TransactionArray list_transaction_by_recurrence(id, page, start, end, _type)
List all transactions created by a recurring transaction.

List all transactions created by a recurring transaction, optionally limited to the date ranges specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the recurring transaction. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD. Both the start and end date must be present.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD. Both the start and end date must be present.  |  |
**_type** | Option<[**crate::models::TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_recurrence

> crate::models::RecurrenceSingle store_recurrence(recurrence_store)
Store a new recurring transaction

Creates a new recurring transaction. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recurrence_store** | [**RecurrenceStore**](RecurrenceStore.md) | JSON array or key=value pairs with the necessary recurring transaction information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::RecurrenceSingle**](RecurrenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_recurrence

> crate::models::RecurrenceSingle update_recurrence(id, recurrence_update)
Update existing recurring transaction.

Update existing recurring transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the recurring transaction. | [required] |
**recurrence_update** | [**RecurrenceUpdate**](RecurrenceUpdate.md) | JSON array with updated recurring transaction information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::RecurrenceSingle**](RecurrenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

