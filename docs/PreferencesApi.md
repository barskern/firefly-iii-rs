# \PreferencesApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_preference**](PreferencesApi.md#get_preference) | **GET** /api/v1/preferences/{name} | Return a single preference.
[**list_preference**](PreferencesApi.md#list_preference) | **GET** /api/v1/preferences | List all users preferences.
[**store_preference**](PreferencesApi.md#store_preference) | **POST** /api/v1/preferences | Store a new preference for this user.
[**update_preference**](PreferencesApi.md#update_preference) | **PUT** /api/v1/preferences/{name} | Update preference



## get_preference

> crate::models::PreferenceSingle get_preference(name)
Return a single preference.

Return a single preference and the value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the preference. | [required] |

### Return type

[**crate::models::PreferenceSingle**](PreferenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_preference

> crate::models::PreferenceArray list_preference(page)
List all users preferences.

List all of the preferences of the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::PreferenceArray**](PreferenceArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_preference

> crate::models::PreferenceSingle store_preference(preference)
Store a new preference for this user.

This endpoint creates a new preference. The name and data are free-format, and entirely up to you. If the preference is not used in Firefly III itself it may not be configurable through the user interface, but you can use this endpoint to persist custom data for your own app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preference** | [**Preference**](Preference.md) | JSON array with the necessary preference information or key=value pairs. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::PreferenceSingle**](PreferenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_preference

> crate::models::PreferenceSingle update_preference(name, preference_update)
Update preference

Update a user's preference.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the preference. Will always overwrite. Will be created if it does not exist. | [required] |
**preference_update** | [**PreferenceUpdate**](PreferenceUpdate.md) | JSON array or key=value pairs with the necessary preference information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::PreferenceSingle**](PreferenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

