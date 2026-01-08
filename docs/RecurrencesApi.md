# \RecurrencesApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_recurrence**](RecurrencesApi.md#delete_recurrence) | **DELETE** /v1/recurrences/{id} | Delete a recurring transaction.
[**get_recurrence**](RecurrencesApi.md#get_recurrence) | **GET** /v1/recurrences/{id} | Get a single recurring transaction.
[**list_recurrence**](RecurrencesApi.md#list_recurrence) | **GET** /v1/recurrences | List all recurring transactions.
[**list_transaction_by_recurrence**](RecurrencesApi.md#list_transaction_by_recurrence) | **GET** /v1/recurrences/{id}/transactions | List all transactions created by a recurring transaction.
[**store_recurrence**](RecurrencesApi.md#store_recurrence) | **POST** /v1/recurrences | Store a new recurring transaction
[**trigger_recurrence_recurrence**](RecurrencesApi.md#trigger_recurrence_recurrence) | **POST** /v1/recurrences/{id}/trigger | Trigger the creation of a transaction for a specific recurring transaction
[**update_recurrence**](RecurrencesApi.md#update_recurrence) | **PUT** /v1/recurrences/{id} | Update existing recurring transaction.



## delete_recurrence

> delete_recurrence(id, x_trace_id)
Delete a recurring transaction.

Delete a recurring transaction. Transactions created by the recurring transaction will not be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the recurring transaction. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recurrence

> models::RecurrenceSingle get_recurrence(id, x_trace_id)
Get a single recurring transaction.

Get a single recurring transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the recurring transaction. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::RecurrenceSingle**](RecurrenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recurrence

> models::RecurrenceArray list_recurrence(x_trace_id, limit, page)
List all recurring transactions.

List all recurring transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::RecurrenceArray**](RecurrenceArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_recurrence

> models::TransactionArray list_transaction_by_recurrence(id, x_trace_id, limit, page, start, end, r#type)
List all transactions created by a recurring transaction.

List all transactions created by a recurring transaction, optionally limited to the date ranges specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the recurring transaction. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD. Both the start and end date must be present.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD. Both the start and end date must be present.  |  |
**r#type** | Option<[**TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned |  |

### Return type

[**models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_recurrence

> models::RecurrenceSingle store_recurrence(recurrence_store, x_trace_id)
Store a new recurring transaction

Creates a new recurring transaction. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recurrence_store** | [**RecurrenceStore**](RecurrenceStore.md) | JSON array or key=value pairs with the necessary recurring transaction information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::RecurrenceSingle**](RecurrenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_recurrence_recurrence

> models::TransactionArray trigger_recurrence_recurrence(id, date, x_trace_id)
Trigger the creation of a transaction for a specific recurring transaction

Trigger the creation of a transaction for a specific recurring transaction. All recurrences have a set of future occurrences. For those moments, you can trigger the creation of the transaction. That means the transaction will be created NOW, instead of on the indicated date. The transaction will be dated to _today_.  So, if you recurring transaction that occurs every Monday, you can trigger the creation of a transaction for Monday in two weeks, today. On that Monday two weeks from now, no transaction will be created. Instead, the transaction is created right now, and dated _today_. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the recurring transaction. | [required] |
**date** | **String** | A date formatted YYYY-MM-DD. This is the date for which you want the recurrence to fire. You can take the date from the list of occurrences in the recurring transaction.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_recurrence

> models::RecurrenceSingle update_recurrence(id, recurrence_update, x_trace_id)
Update existing recurring transaction.

Update existing recurring transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the recurring transaction. | [required] |
**recurrence_update** | [**RecurrenceUpdate**](RecurrenceUpdate.md) | JSON array with updated recurring transaction information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::RecurrenceSingle**](RecurrenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

