# \TagsApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tag**](TagsApi.md#delete_tag) | **DELETE** /api/v1/tags/{tag} | Delete an tag.
[**get_tag**](TagsApi.md#get_tag) | **GET** /api/v1/tags/{tag} | Get a single tag.
[**list_attachment_by_tag**](TagsApi.md#list_attachment_by_tag) | **GET** /api/v1/tags/{tag}/attachments | Lists all attachments.
[**list_tag**](TagsApi.md#list_tag) | **GET** /api/v1/tags | List all tags.
[**list_transaction_by_tag**](TagsApi.md#list_transaction_by_tag) | **GET** /api/v1/tags/{tag}/transactions | List all transactions with this tag.
[**store_tag**](TagsApi.md#store_tag) | **POST** /api/v1/tags | Store a new tag
[**update_tag**](TagsApi.md#update_tag) | **PUT** /api/v1/tags/{tag} | Update existing tag.



## delete_tag

> delete_tag(tag)
Delete an tag.

Delete an tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | Either the tag itself or the tag ID. If you use the tag itself, and it contains international (non-ASCII) characters, your milage may vary. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag

> crate::models::TagSingle get_tag(tag, page)
Get a single tag.

Get a single tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | Either the tag itself or the tag ID. If you use the tag itself, and it contains international (non-ASCII) characters, your milage may vary. | [required] |
**page** | Option<**i32**> | Page number |  |

### Return type

[**crate::models::TagSingle**](TagSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment_by_tag

> crate::models::AttachmentArray list_attachment_by_tag(tag, page)
Lists all attachments.

Lists all attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | Either the tag itself or the tag ID. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::AttachmentArray**](AttachmentArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tag

> crate::models::TagArray list_tag(page)
List all tags.

List all of the user's tags.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number |  |

### Return type

[**crate::models::TagArray**](TagArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_tag

> crate::models::TransactionArray list_transaction_by_tag(tag, page, start, end, _type)
List all transactions with this tag.

List all transactions with this tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | Either the tag itself or the tag ID. | [required] |
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
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_tag

> crate::models::TagSingle store_tag(tag_model_store)
Store a new tag

Creates a new tag. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_model_store** | [**TagModelStore**](TagModelStore.md) | JSON array or key=value pairs with the necessary tag information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::TagSingle**](TagSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tag

> crate::models::TagSingle update_tag(tag, tag_model_update)
Update existing tag.

Update existing tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | Either the tag itself or the tag ID. If you use the tag itself, and it contains international (non-ASCII) characters, your milage may vary. | [required] |
**tag_model_update** | [**TagModelUpdate**](TagModelUpdate.md) | JSON array with updated tag information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::TagSingle**](TagSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

