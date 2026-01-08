# \UsersApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /v1/users/{id} | Delete a user.
[**get_user**](UsersApi.md#get_user) | **GET** /v1/users/{id} | Get a single user.
[**list_user**](UsersApi.md#list_user) | **GET** /v1/users | List all users.
[**store_user**](UsersApi.md#store_user) | **POST** /v1/users | Store a new user
[**update_user**](UsersApi.md#update_user) | **PUT** /v1/users/{id} | Update an existing user's information.



## delete_user

> delete_user(id, x_trace_id)
Delete a user.

Delete a user. You cannot delete the user you're authenticated with. This cannot be undone. Be careful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user ID. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::UserSingle get_user(id, x_trace_id)
Get a single user.

Gets all info of a single user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user ID. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::UserSingle**](UserSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user

> models::UserArray list_user(x_trace_id, limit, page)
List all users.

List all the users in this instance of Firefly III.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::UserArray**](UserArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_user

> models::UserSingle store_user(user, x_trace_id)
Store a new user

Creates a new user. The data required can be submitted as a JSON body or as a list of parameters. The user will be given a random password, which they can reset using the \"forgot password\" function. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | [**User**](User.md) | JSON array or key=value pairs with the necessary user information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::UserSingle**](UserSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::UserSingle update_user(id, user, x_trace_id)
Update an existing user's information.

Update existing user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user ID. | [required] |
**user** | [**User**](User.md) | JSON array with updated user information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::UserSingle**](UserSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

