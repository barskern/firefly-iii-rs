# \PreferencesApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_preference**](PreferencesApi.md#get_preference) | **GET** /api/v1/preferences/{name} | Return a single preference.
[**list_preference**](PreferencesApi.md#list_preference) | **GET** /api/v1/preferences | List all users preferences.
[**update_preference**](PreferencesApi.md#update_preference) | **PUT** /api/v1/preferences/{name} | Update preference



## get_preference

> crate::models::PreferenceSingle get_preference(name)
Return a single preference.

Return a single preference.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_preference

> crate::models::PreferenceArray list_preference(page)
List all users preferences.

List all preferences of the user.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_preference

> crate::models::PreferenceSingle update_preference(name, preference)
Update preference

Update a user's preference.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the preference. Will always overwrite. Will be created if it does not exist. | [required] |
**preference** | [**Preference**](Preference.md) | JSON array or key=value pairs with the necessary preference information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::PreferenceSingle**](PreferenceSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

