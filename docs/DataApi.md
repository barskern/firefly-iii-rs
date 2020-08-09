# \DataApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**destroy_data**](DataApi.md#destroy_data) | **DELETE** /api/v1/data/destroy | Endpoint to destroy user data



## destroy_data

> destroy_data(objects)
Endpoint to destroy user data

A call to this endpoint permanently destroys the requested data type. Use it with care and always with user permission. The demo user is incapable of using this endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**objects** | [**crate::models::DataDestroyObject**](.md) | The type of data that you wish to destroy. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

