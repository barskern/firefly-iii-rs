# \PiggyBanksApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_piggy_bank**](PiggyBanksApi.md#delete_piggy_bank) | **DELETE** /v1/piggy-banks/{id} | Delete a piggy bank.
[**get_piggy_bank**](PiggyBanksApi.md#get_piggy_bank) | **GET** /v1/piggy-banks/{id} | Get a single piggy bank.
[**list_attachment_by_piggy_bank**](PiggyBanksApi.md#list_attachment_by_piggy_bank) | **GET** /v1/piggy-banks/{id}/attachments | Lists all attachments.
[**list_event_by_piggy_bank**](PiggyBanksApi.md#list_event_by_piggy_bank) | **GET** /v1/piggy-banks/{id}/events | List all events linked to a piggy bank.
[**list_piggy_bank**](PiggyBanksApi.md#list_piggy_bank) | **GET** /v1/piggy-banks | List all piggy banks.
[**store_piggy_bank**](PiggyBanksApi.md#store_piggy_bank) | **POST** /v1/piggy-banks | Store a new piggy bank
[**update_piggy_bank**](PiggyBanksApi.md#update_piggy_bank) | **PUT** /v1/piggy-banks/{id} | Update existing piggy bank.



## delete_piggy_bank

> delete_piggy_bank(id, x_trace_id)
Delete a piggy bank.

Delete a piggy bank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the piggy bank. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_piggy_bank

> models::PiggyBankSingle get_piggy_bank(id, x_trace_id)
Get a single piggy bank.

Get a single piggy bank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the piggy bank. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::PiggyBankSingle**](PiggyBankSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment_by_piggy_bank

> models::AttachmentArray list_attachment_by_piggy_bank(id, x_trace_id, limit, page)
Lists all attachments.

Lists all attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the piggy bank. | [required] |
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


## list_event_by_piggy_bank

> models::PiggyBankEventArray list_event_by_piggy_bank(id, x_trace_id, limit, page)
List all events linked to a piggy bank.

List all events linked to a piggy bank (adding and removing money).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the piggy bank | [required] |
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


## list_piggy_bank

> models::PiggyBankArray list_piggy_bank(x_trace_id, limit, page)
List all piggy banks.

List all piggy banks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::PiggyBankArray**](PiggyBankArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_piggy_bank

> models::PiggyBankSingle store_piggy_bank(piggy_bank_store, x_trace_id)
Store a new piggy bank

Creates a new piggy bank. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**piggy_bank_store** | [**PiggyBankStore**](PiggyBankStore.md) | JSON array or key=value pairs with the necessary piggy bank information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::PiggyBankSingle**](PiggyBankSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_piggy_bank

> models::PiggyBankSingle update_piggy_bank(id, piggy_bank_update, x_trace_id)
Update existing piggy bank.

Update existing piggy bank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the piggy bank | [required] |
**piggy_bank_update** | [**PiggyBankUpdate**](PiggyBankUpdate.md) | JSON array with updated piggy bank information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::PiggyBankSingle**](PiggyBankSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

