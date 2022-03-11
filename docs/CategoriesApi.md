# \CategoriesApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_category**](CategoriesApi.md#delete_category) | **DELETE** /api/v1/categories/{id} | Delete a category.
[**get_category**](CategoriesApi.md#get_category) | **GET** /api/v1/categories/{id} | Get a single category.
[**list_attachment_by_category**](CategoriesApi.md#list_attachment_by_category) | **GET** /api/v1/categories/{id}/attachments | Lists all attachments.
[**list_category**](CategoriesApi.md#list_category) | **GET** /api/v1/categories | List all categories.
[**list_transaction_by_category**](CategoriesApi.md#list_transaction_by_category) | **GET** /api/v1/categories/{id}/transactions | List all transactions in a category.
[**store_category**](CategoriesApi.md#store_category) | **POST** /api/v1/categories | Store a new category
[**update_category**](CategoriesApi.md#update_category) | **PUT** /api/v1/categories/{id} | Update existing category.



## delete_category

> delete_category(id)
Delete a category.

Delete a category. Transactions will not be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the category. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_category

> crate::models::CategorySingle get_category(id, start, end)
Get a single category.

Get a single category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the category. | [required] |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to show spent and earned info.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to show spent and earned info.  |  |

### Return type

[**crate::models::CategorySingle**](CategorySingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment_by_category

> crate::models::AttachmentArray list_attachment_by_category(id, page)
Lists all attachments.

Lists all attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the category. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::AttachmentArray**](AttachmentArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_category

> crate::models::CategoryArray list_category(page)
List all categories.

List all categories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::CategoryArray**](CategoryArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_category

> crate::models::TransactionArray list_transaction_by_category(id, page, start, end, _type)
List all transactions in a category.

List all transactions in a category, optionally limited to the date ranges specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the category. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is per 50. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the result list.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the result list.  |  |
**_type** | Option<[**crate::models::TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_category

> crate::models::CategorySingle store_category(category)
Store a new category

Creates a new category. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | [**Category**](Category.md) | JSON array or key=value pairs with the necessary category information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::CategorySingle**](CategorySingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_category

> crate::models::CategorySingle update_category(id, category_update)
Update existing category.

Update existing category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the category. | [required] |
**category_update** | [**CategoryUpdate**](CategoryUpdate.md) | JSON array with updated category information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::CategorySingle**](CategorySingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

