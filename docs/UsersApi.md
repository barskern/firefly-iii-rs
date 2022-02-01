# \UsersApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /api/v1/users/{id} | Delete a user.
[**get_user**](UsersApi.md#get_user) | **GET** /api/v1/users/{id} | Get a single user.
[**list_user**](UsersApi.md#list_user) | **GET** /api/v1/users | List all users.
[**store_user**](UsersApi.md#store_user) | **POST** /api/v1/users | Store a new user
[**update_user**](UsersApi.md#update_user) | **PUT** /api/v1/users/{id} | Update an existing user's information.



## delete_user

> delete_user(id)
Delete a user.

Delete a user. You cannot delete the user you're authenticated with. This cannot be undone. Be careful!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user ID. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::UserSingle get_user(id)
Get a single user.

Gets all info of a single user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user ID. | [required] |

### Return type

[**crate::models::UserSingle**](UserSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user

> crate::models::UserArray list_user(page)
List all users.

List all the users in this instance of Firefly III.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number, if necessary. The default pagination is 50, so 50 users per page. |  |

### Return type

[**crate::models::UserArray**](UserArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_user

> crate::models::UserSingle store_user(user)
Store a new user

Creates a new user. The data required can be submitted as a JSON body or as a list of parameters. The user will be given a random password, which they can reset using the \"forgot password\" function. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | [**User**](User.md) | JSON array or key=value pairs with the necessary user information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::UserSingle**](UserSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::UserSingle update_user(id, user)
Update an existing user's information.

Update existing user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user ID. | [required] |
**user** | [**User**](User.md) | JSON array with updated user information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::UserSingle**](UserSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

