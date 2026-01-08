# \ObjectGroupsApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_group**](ObjectGroupsApi.md#delete_object_group) | **DELETE** /v1/object-groups/{id} | Delete a object group.
[**get_object_group**](ObjectGroupsApi.md#get_object_group) | **GET** /v1/object-groups/{id} | Get a single object group.
[**list_bill_by_object_group**](ObjectGroupsApi.md#list_bill_by_object_group) | **GET** /v1/object-groups/{id}/bills | List all bills with this object group.
[**list_object_groups**](ObjectGroupsApi.md#list_object_groups) | **GET** /v1/object-groups | List all object groups.
[**list_piggy_bank_by_object_group**](ObjectGroupsApi.md#list_piggy_bank_by_object_group) | **GET** /v1/object-groups/{id}/piggy-banks | List all piggy banks related to the object group.
[**update_object_group**](ObjectGroupsApi.md#update_object_group) | **PUT** /v1/object-groups/{id} | Update existing object group.



## delete_object_group

> delete_object_group(id, x_trace_id)
Delete a object group.

Delete a object group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the object group. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_group

> models::ObjectGroupSingle get_object_group(id, x_trace_id)
Get a single object group.

Get a single object group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the object group. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::ObjectGroupSingle**](ObjectGroupSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_bill_by_object_group

> models::BillArray list_bill_by_object_group(id, x_trace_id, limit, page)
List all bills with this object group.

List all bills with this object group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::BillArray**](BillArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_object_groups

> models::ObjectGroupArray list_object_groups(x_trace_id, limit, page)
List all object groups.

List all object groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::ObjectGroupArray**](ObjectGroupArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_piggy_bank_by_object_group

> models::PiggyBankArray list_piggy_bank_by_object_group(id, x_trace_id, limit, page)
List all piggy banks related to the object group.

This endpoint returns a list of all the piggy banks connected to the object group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
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


## update_object_group

> models::ObjectGroupSingle update_object_group(id, object_group_update, x_trace_id)
Update existing object group.

Update existing object group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the object group | [required] |
**object_group_update** | [**ObjectGroupUpdate**](ObjectGroupUpdate.md) | JSON array with updated piggy bank information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::ObjectGroupSingle**](ObjectGroupSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

