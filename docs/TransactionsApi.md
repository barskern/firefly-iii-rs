# \TransactionsApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_transaction**](TransactionsApi.md#delete_transaction) | **DELETE** /v1/transactions/{id} | Delete a transaction.
[**delete_transaction_journal**](TransactionsApi.md#delete_transaction_journal) | **DELETE** /v1/transaction-journals/{id} | Delete split from transaction
[**get_transaction**](TransactionsApi.md#get_transaction) | **GET** /v1/transactions/{id} | Get a single transaction.
[**get_transaction_by_journal**](TransactionsApi.md#get_transaction_by_journal) | **GET** /v1/transaction-journals/{id} | Get a single transaction, based on one of the underlying transaction journals (transaction splits).
[**list_attachment_by_transaction**](TransactionsApi.md#list_attachment_by_transaction) | **GET** /v1/transactions/{id}/attachments | Lists all attachments.
[**list_event_by_transaction**](TransactionsApi.md#list_event_by_transaction) | **GET** /v1/transactions/{id}/piggy-bank-events | Lists all piggy bank events.
[**list_links_by_journal**](TransactionsApi.md#list_links_by_journal) | **GET** /v1/transaction-journals/{id}/links | Lists all the transaction links for an individual journal (individual split).
[**list_transaction**](TransactionsApi.md#list_transaction) | **GET** /v1/transactions | List all the user's transactions. 
[**store_transaction**](TransactionsApi.md#store_transaction) | **POST** /v1/transactions | Store a new transaction
[**update_transaction**](TransactionsApi.md#update_transaction) | **PUT** /v1/transactions/{id} | Update existing transaction. For more information, see https://docs.firefly-iii.org/references/firefly-iii/api/specials/



## delete_transaction

> delete_transaction(id, x_trace_id)
Delete a transaction.

Delete a transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transaction_journal

> delete_transaction_journal(id, x_trace_id)
Delete split from transaction

Delete an individual journal (split) from a transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction journal (the split) you wish to delete. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction

> models::TransactionSingle get_transaction(id, x_trace_id)
Get a single transaction.

Get a single transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::TransactionSingle**](TransactionSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_by_journal

> models::TransactionSingle get_transaction_by_journal(id, x_trace_id)
Get a single transaction, based on one of the underlying transaction journals (transaction splits).

Get a single transaction by underlying journal (split).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction journal (split). | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::TransactionSingle**](TransactionSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment_by_transaction

> models::AttachmentArray list_attachment_by_transaction(id, x_trace_id, limit, page)
Lists all attachments.

Lists all attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::AttachmentArray**](AttachmentArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_by_transaction

> models::PiggyBankEventArray list_event_by_transaction(id, x_trace_id, limit, page)
Lists all piggy bank events.

Lists all piggy bank events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::PiggyBankEventArray**](PiggyBankEventArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_links_by_journal

> models::TransactionLinkArray list_links_by_journal(id, x_trace_id, limit, page)
Lists all the transaction links for an individual journal (individual split).

Lists all the transaction links for an individual journal (a split). Don't use the group ID, you need the actual underlying journal (the split).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction journal / the split. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::TransactionLinkArray**](TransactionLinkArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction

> models::TransactionArray list_transaction(x_trace_id, limit, page, start, end, r#type)
List all the user's transactions. 

List all the user's transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD. This is the start date of the selected range (inclusive).  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD. This is the end date of the selected range (inclusive).  |  |
**r#type** | Option<[**TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned. |  |

### Return type

[**models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_transaction

> models::TransactionSingle store_transaction(transaction_store, x_trace_id)
Store a new transaction

Creates a new transaction. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_store** | [**TransactionStore**](TransactionStore.md) | JSON array or key=value pairs with the necessary transaction information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::TransactionSingle**](TransactionSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transaction

> models::TransactionSingle update_transaction(id, transaction_update, x_trace_id)
Update existing transaction. For more information, see https://docs.firefly-iii.org/references/firefly-iii/api/specials/

Update an existing transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction. | [required] |
**transaction_update** | [**TransactionUpdate**](TransactionUpdate.md) | JSON array with updated transaction information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::TransactionSingle**](TransactionSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

