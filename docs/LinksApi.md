# \LinksApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_link_type**](LinksApi.md#delete_link_type) | **DELETE** /api/v1/link_types/{id} | Permanently delete link type.
[**delete_transaction_link**](LinksApi.md#delete_transaction_link) | **DELETE** /api/v1/transaction_links/{id} | Permanently delete link between transactions.
[**get_link_type**](LinksApi.md#get_link_type) | **GET** /api/v1/link_types/{id} | Get single a link type.
[**get_transaction_link**](LinksApi.md#get_transaction_link) | **GET** /api/v1/transaction_links/{id} | Get a single link.
[**list_link_type**](LinksApi.md#list_link_type) | **GET** /api/v1/link_types | List all types of links.
[**list_transaction_by_link_type**](LinksApi.md#list_transaction_by_link_type) | **GET** /api/v1/link_types/{id}/transactions | List all transactions under this link type.
[**list_transaction_link**](LinksApi.md#list_transaction_link) | **GET** /api/v1/transaction_links | List all transaction links.
[**store_link_type**](LinksApi.md#store_link_type) | **POST** /api/v1/link_types | Create a new link type
[**store_transaction_link**](LinksApi.md#store_transaction_link) | **POST** /api/v1/transaction_links | Create a new link between transactions
[**update_link_type**](LinksApi.md#update_link_type) | **PUT** /api/v1/link_types/{id} | Update existing link type.
[**update_transaction_link**](LinksApi.md#update_transaction_link) | **PUT** /api/v1/transaction_links/{id} | Update an existing link between transactions.



## delete_link_type

> delete_link_type(id)
Permanently delete link type.

Will permanently delete a link type. The links between transactions will be removed. The transactions themselves remain. You cannot delete some of the system provided link types, indicated by the editable=false flag when you list it. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the link type. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transaction_link

> delete_transaction_link(id)
Permanently delete link between transactions.

Will permanently delete link. Transactions remain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction link. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_link_type

> crate::models::LinkTypeSingle get_link_type(id)
Get single a link type.

Returns a single link type by its ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the link type. | [required] |

### Return type

[**crate::models::LinkTypeSingle**](LinkTypeSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_link

> crate::models::TransactionLinkSingle get_transaction_link(id)
Get a single link.

Returns a single link by its ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction link. | [required] |

### Return type

[**crate::models::TransactionLinkSingle**](TransactionLinkSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_link_type

> crate::models::LinkTypeArray list_link_type(page)
List all types of links.

List all the link types the system has. These include the default ones as well as any new ones. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50 items. |  |

### Return type

[**crate::models::LinkTypeArray**](LinkTypeArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_link_type

> crate::models::TransactionArray list_transaction_by_link_type(id, page, start, end, _type)
List all transactions under this link type.

List all transactions under this link type, both the inward and outward transactions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the link type. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the results.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the results.  |  |
**_type** | Option<[**crate::models::TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned. |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_link

> crate::models::TransactionLinkArray list_transaction_link(page)
List all transaction links.

List all the transaction links. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**crate::models::TransactionLinkArray**](TransactionLinkArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_link_type

> crate::models::LinkTypeSingle store_link_type(link_type)
Create a new link type

Creates a new link type. The data required can be submitted as a JSON body or as a list of parameters (in key=value pairs, like a webform).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_type** | [**LinkType**](LinkType.md) | JSON array with the necessary link type information or key=value pairs. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::LinkTypeSingle**](LinkTypeSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_transaction_link

> crate::models::TransactionLinkSingle store_transaction_link(transaction_link_store)
Create a new link between transactions

Store a new link between two transactions. For this end point you need the journal_id from a transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_link_store** | [**TransactionLinkStore**](TransactionLinkStore.md) | JSON array with the necessary link type information or key=value pairs. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::TransactionLinkSingle**](TransactionLinkSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_link_type

> crate::models::LinkTypeSingle update_link_type(id, link_type_update)
Update existing link type.

Used to update a single link type. All fields that are not submitted will be cleared (set to NULL). The model will tell you which fields are mandatory. You cannot update some of the system provided link types, indicated by the editable=false flag when you list it. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the link type. | [required] |
**link_type_update** | [**LinkTypeUpdate**](LinkTypeUpdate.md) | JSON array or formdata with updated link type information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::LinkTypeSingle**](LinkTypeSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transaction_link

> crate::models::TransactionLinkSingle update_transaction_link(id, transaction_link_update)
Update an existing link between transactions.

Used to update a single existing link. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the transaction link. | [required] |
**transaction_link_update** | [**TransactionLinkUpdate**](TransactionLinkUpdate.md) | JSON array or formdata with updated link type information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::TransactionLinkSingle**](TransactionLinkSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

