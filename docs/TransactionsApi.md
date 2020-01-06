# \TransactionsApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_transaction**](TransactionsApi.md#delete_transaction) | **DELETE** /api/v1/transactions/{id} | Delete a transaction.
[**get_transaction**](TransactionsApi.md#get_transaction) | **GET** /api/v1/transactions/{id} | Get a single transaction.
[**get_transaction_by_journal**](TransactionsApi.md#get_transaction_by_journal) | **GET** /api/v1/transaction-journals/{id} | Get a single transaction, based on one of the underlying transaction journals.
[**list_attachment_by_transaction**](TransactionsApi.md#list_attachment_by_transaction) | **GET** /api/v1/transactions/{id}/attachments | Lists all attachments.
[**list_event_by_transaction**](TransactionsApi.md#list_event_by_transaction) | **GET** /api/v1/transactions/{id}/piggy_bank_events | Lists all piggy bank events.
[**list_transaction**](TransactionsApi.md#list_transaction) | **GET** /api/v1/transactions | List all the user's transactions. 
[**store_transaction**](TransactionsApi.md#store_transaction) | **POST** /api/v1/transactions | Store a new transaction
[**update_transaction**](TransactionsApi.md#update_transaction) | **PUT** /api/v1/transactions/{id} | Update existing transaction.



## delete_transaction

> delete_transaction(id)
Delete a transaction.

Delete a transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the transaction. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction

> crate::models::TransactionSingle get_transaction(id)
Get a single transaction.

Get a single transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the transaction. | [required] |

### Return type

[**crate::models::TransactionSingle**](TransactionSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_by_journal

> crate::models::TransactionSingle get_transaction_by_journal(id)
Get a single transaction, based on one of the underlying transaction journals.

Get a single transaction by underlying journal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the transaction journal. | [required] |

### Return type

[**crate::models::TransactionSingle**](TransactionSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment_by_transaction

> crate::models::AttachmentArray list_attachment_by_transaction(id, page)
Lists all attachments.

Lists all attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the transaction. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::AttachmentArray**](AttachmentArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_by_transaction

> crate::models::PiggyBankEventArray list_event_by_transaction(id, page)
Lists all piggy bank events.

Lists all piggy bank events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the transaction. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::PiggyBankEventArray**](PiggyBankEventArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction

> crate::models::TransactionArray list_transaction(page, start, end, _type)
List all the user's transactions. 

List all the user's transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD. This is the start date of the selected range (inclusive).  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD. This is the end date of the selected range (inclusive).  |  |
**_type** | Option<[**crate::models::TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned. |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_transaction

> crate::models::TransactionSingle store_transaction(transaction)
Store a new transaction

Creates a new transaction. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction** | [**Transaction**](Transaction.md) | JSON array or key=value pairs with the necessary transaction information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::TransactionSingle**](TransactionSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transaction

> crate::models::TransactionSingle update_transaction(id, transaction)
Update existing transaction.

Update an existing transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the transaction. | [required] |
**transaction** | [**Transaction**](Transaction.md) | JSON array with updated transaction information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::TransactionSingle**](TransactionSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

