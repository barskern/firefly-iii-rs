# \LinksApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_link_type**](LinksApi.md#delete_link_type) | **DELETE** /v1/link-types/{id} | Permanently delete link type.
[**delete_transaction_link**](LinksApi.md#delete_transaction_link) | **DELETE** /v1/transaction-links/{id} | Permanently delete link between transactions.
[**get_link_type**](LinksApi.md#get_link_type) | **GET** /v1/link-types/{id} | Get single a link type.
[**get_transaction_link**](LinksApi.md#get_transaction_link) | **GET** /v1/transaction-links/{id} | Get a single link.
[**list_link_type**](LinksApi.md#list_link_type) | **GET** /v1/link-types | List all types of links.
[**list_transaction_by_link_type**](LinksApi.md#list_transaction_by_link_type) | **GET** /v1/link-types/{id}/transactions | List all transactions under this link type.
[**list_transaction_link**](LinksApi.md#list_transaction_link) | **GET** /v1/transaction-links | List all transaction links.
[**store_link_type**](LinksApi.md#store_link_type) | **POST** /v1/link-types | Create a new link type
[**store_transaction_link**](LinksApi.md#store_transaction_link) | **POST** /v1/transaction-links | Create a new link between transactions
[**update_link_type**](LinksApi.md#update_link_type) | **PUT** /v1/link-types/{id} | Update existing link type.
[**update_transaction_link**](LinksApi.md#update_transaction_link) | **PUT** /v1/transaction-links/{id} | Update an existing link between transactions.



## delete_link_type

> delete_link_type(id, x_trace_id)
Permanently delete link type.

Will permanently delete a link type. The links between transactions will be removed. The transactions themselves remain. You cannot delete some of the system provided link types, indicated by the editable=false flag when you list it. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the link type. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transaction_link

> delete_transaction_link(id, x_trace_id)
Permanently delete link between transactions.

Will permanently delete link. Transactions remain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction link. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_link_type

> models::LinkTypeSingle get_link_type(id, x_trace_id)
Get single a link type.

Returns a single link type by its ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the link type. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::LinkTypeSingle**](LinkTypeSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_link

> models::TransactionLinkSingle get_transaction_link(id, x_trace_id)
Get a single link.

Returns a single link by its ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction link. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::TransactionLinkSingle**](TransactionLinkSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_link_type

> models::LinkTypeArray list_link_type(x_trace_id, limit, page)
List all types of links.

List all the link types the system has. These include the default ones as well as any new ones. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::LinkTypeArray**](LinkTypeArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_link_type

> models::TransactionArray list_transaction_by_link_type(id, x_trace_id, limit, page, start, end, r#type)
List all transactions under this link type.

List all transactions under this link type, both the inward and outward transactions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the link type. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the results.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the results.  |  |
**r#type** | Option<[**TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned. |  |

### Return type

[**models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_link

> models::TransactionLinkArray list_transaction_link(x_trace_id, limit, page)
List all transaction links.

List all the transaction links. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## store_link_type

> models::LinkTypeSingle store_link_type(link_type, x_trace_id)
Create a new link type

Creates a new link type. The data required can be submitted as a JSON body or as a list of parameters (in key=value pairs, like a webform).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_type** | [**LinkType**](LinkType.md) | JSON array with the necessary link type information or key=value pairs. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::LinkTypeSingle**](LinkTypeSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_transaction_link

> models::TransactionLinkSingle store_transaction_link(transaction_link_store, x_trace_id)
Create a new link between transactions

Store a new link between two transactions. For this end point you need the journal_id from a transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_link_store** | [**TransactionLinkStore**](TransactionLinkStore.md) | JSON array with the necessary link type information or key=value pairs. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::TransactionLinkSingle**](TransactionLinkSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_link_type

> models::LinkTypeSingle update_link_type(id, link_type_update, x_trace_id)
Update existing link type.

Used to update a single link type. All fields that are not submitted will be cleared (set to NULL). The model will tell you which fields are mandatory. You cannot update some of the system provided link types, indicated by the editable=false flag when you list it. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the link type. | [required] |
**link_type_update** | [**LinkTypeUpdate**](LinkTypeUpdate.md) | JSON array or form-data with updated link type information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::LinkTypeSingle**](LinkTypeSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transaction_link

> models::TransactionLinkSingle update_transaction_link(id, transaction_link_update, x_trace_id)
Update an existing link between transactions.

Used to update a single existing link. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction link. | [required] |
**transaction_link_update** | [**TransactionLinkUpdate**](TransactionLinkUpdate.md) | JSON array or form-data with updated link type information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::TransactionLinkSingle**](TransactionLinkSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

