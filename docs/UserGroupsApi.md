# \UserGroupsApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_group**](UserGroupsApi.md#get_user_group) | **GET** /v1/user-groups/{id} | Get a single user group.
[**list_user_groups**](UserGroupsApi.md#list_user_groups) | **GET** /v1/user-groups | List all the user groups available to this user. 
[**update_user_group**](UserGroupsApi.md#update_user_group) | **PUT** /v1/user-groups/{id} | Update an existing user group.



## get_user_group

> models::UserGroupSingle get_user_group(id, x_trace_id)
Get a single user group.

Returns a single user group by its ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the user group. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::UserGroupSingle**](UserGroupSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_groups

> models::UserGroupArray list_user_groups(x_trace_id, limit, page)
List all the user groups available to this user. 

List all the user groups available to this user. These are essentially the 'financial administrations' that Firefly III supports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::UserGroupArray**](UserGroupArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_group

> models::UserGroupSingle update_user_group(id, user_group_update, x_trace_id)
Update an existing user group.

Used to update a single user group. The available fields are still limited. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
**user_group_update** | [**UserGroupUpdate**](UserGroupUpdate.md) | JSON array or form-data with new user group information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::UserGroupSingle**](UserGroupSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

