# \ConfigurationApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_configuration**](ConfigurationApi.md#get_configuration) | **GET** /v1/configuration | Get Firefly III system configuration values.
[**get_single_configuration**](ConfigurationApi.md#get_single_configuration) | **GET** /v1/configuration/{name} | Get a single Firefly III system configuration value
[**set_configuration**](ConfigurationApi.md#set_configuration) | **PUT** /v1/configuration/{name} | Update configuration value



## get_configuration

> Vec<models::Configuration> get_configuration(x_trace_id)
Get Firefly III system configuration values.

Returns all editable and not-editable configuration values for this Firefly III installation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**Vec<models::Configuration>**](Configuration.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-www-form-urlencoded

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_single_configuration

> models::ConfigurationSingle get_single_configuration(name, x_trace_id)
Get a single Firefly III system configuration value

Returns one configuration variable for this Firefly III installation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**ConfigValueFilter**](.md) | The name of the configuration value you want to know. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::ConfigurationSingle**](ConfigurationSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_configuration

> models::ConfigurationSingle set_configuration(name, value, x_trace_id)
Update configuration value

Set a single configuration value. Not all configuration values can be updated so the list of accepted configuration variables is small.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**ConfigValueUpdateFilter**](.md) | The name of the configuration value you want to update. | [required] |
**value** | [**models::PolymorphicProperty**](PolymorphicProperty.md) |  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::ConfigurationSingle**](ConfigurationSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

