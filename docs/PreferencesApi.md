# \PreferencesApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_preference**](PreferencesApi.md#get_preference) | **GET** /v1/preferences/{name} | Return a single preference.
[**list_preference**](PreferencesApi.md#list_preference) | **GET** /v1/preferences | List all users preferences.
[**store_preference**](PreferencesApi.md#store_preference) | **POST** /v1/preferences | Store a new preference for this user.
[**update_preference**](PreferencesApi.md#update_preference) | **PUT** /v1/preferences/{name} | Update preference



## get_preference

> models::PreferenceSingle get_preference(name, x_trace_id)
Return a single preference.

Return a single preference and the value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the preference. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::PreferenceSingle**](PreferenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_preference

> models::PreferenceArray list_preference(x_trace_id, limit, page)
List all users preferences.

List all of the preferences of the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::PreferenceArray**](PreferenceArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_preference

> models::PreferenceSingle store_preference(preference, x_trace_id)
Store a new preference for this user.

This endpoint creates a new preference. The name and data are free-format, and entirely up to you. If the preference is not used in Firefly III itself it may not be configurable through the user interface, but you can use this endpoint to persist custom data for your own app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preference** | [**Preference**](Preference.md) | JSON array with the necessary preference information or key=value pairs. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::PreferenceSingle**](PreferenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_preference

> models::PreferenceSingle update_preference(name, preference_update, x_trace_id)
Update preference

Update a user's preference.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the preference. Will always overwrite. Will be created if it does not exist. | [required] |
**preference_update** | [**PreferenceUpdate**](PreferenceUpdate.md) | JSON array or key=value pairs with the necessary preference information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::PreferenceSingle**](PreferenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

