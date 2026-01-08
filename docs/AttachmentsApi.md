# \AttachmentsApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_attachment**](AttachmentsApi.md#delete_attachment) | **DELETE** /v1/attachments/{id} | Delete an attachment.
[**download_attachment**](AttachmentsApi.md#download_attachment) | **GET** /v1/attachments/{id}/download | Download a single attachment.
[**get_attachment**](AttachmentsApi.md#get_attachment) | **GET** /v1/attachments/{id} | Get a single attachment.
[**list_attachment**](AttachmentsApi.md#list_attachment) | **GET** /v1/attachments | List all attachments.
[**store_attachment**](AttachmentsApi.md#store_attachment) | **POST** /v1/attachments | Store a new attachment.
[**update_attachment**](AttachmentsApi.md#update_attachment) | **PUT** /v1/attachments/{id} | Update existing attachment.
[**upload_attachment**](AttachmentsApi.md#upload_attachment) | **POST** /v1/attachments/{id}/upload | Upload an attachment.



## delete_attachment

> delete_attachment(id, x_trace_id)
Delete an attachment.

With this endpoint you delete an attachment, including any stored file data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the single attachment. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_attachment

> std::path::PathBuf download_attachment(id, x_trace_id)
Download a single attachment.

This endpoint allows you to download the binary content of a transaction. It will be sent to you as a download, using the content type \"application/octet-stream\" and content disposition \"attachment; filename=example.pdf\". 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the attachment. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attachment

> models::AttachmentSingle get_attachment(id, x_trace_id)
Get a single attachment.

Get a single attachment. This endpoint only returns the available metadata for the attachment. Actual file data is handled in two other endpoints (see below). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the attachment. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::AttachmentSingle**](AttachmentSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment

> models::AttachmentArray list_attachment(x_trace_id, limit, page)
List all attachments.

This endpoint lists all attachments. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## store_attachment

> models::AttachmentSingle store_attachment(attachment_store, x_trace_id)
Store a new attachment.

Creates a new attachment. The data required can be submitted as a JSON body or as a list of parameters. You cannot use this endpoint to upload the actual file data (see below). This endpoint only creates the attachment object. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_store** | [**AttachmentStore**](AttachmentStore.md) | JSON array or key=value pairs with the necessary attachment information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::AttachmentSingle**](AttachmentSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_attachment

> models::AttachmentSingle update_attachment(id, attachment_update, x_trace_id)
Update existing attachment.

Update the meta data for an existing attachment. This endpoint does not allow you to upload or download data. For that, see below. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the attachment. | [required] |
**attachment_update** | [**AttachmentUpdate**](AttachmentUpdate.md) | JSON array with updated attachment information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::AttachmentSingle**](AttachmentSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_attachment

> upload_attachment(id, x_trace_id, body)
Upload an attachment.

Use this endpoint to upload (and possible overwrite) the file contents of an attachment. Simply put the entire file in the body as binary data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the attachment. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

