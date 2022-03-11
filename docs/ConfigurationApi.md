# \ConfigurationApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_configuration**](ConfigurationApi.md#get_configuration) | **GET** /api/v1/configuration | Get Firefly III system configuration values.



## get_configuration

> Vec<crate::models::Configuration> get_configuration()
Get Firefly III system configuration values.

Returns all editable and not-editable configuration values for this Firefly III installation

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Configuration>**](Configuration.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

