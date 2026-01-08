# \AboutApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_about**](AboutApi.md#get_about) | **GET** /v1/about | System information end point.
[**get_cron**](AboutApi.md#get_cron) | **GET** /v1/cron/{cliToken} | Cron job endpoint
[**get_current_user**](AboutApi.md#get_current_user) | **GET** /v1/about/user | Currently authenticated user endpoint.



## get_about

> models::SystemInfo get_about(x_trace_id)
System information end point.

Returns general system information and versions of the (supporting) software. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::SystemInfo**](SystemInfo.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cron

> models::CronResult get_cron(cli_token, x_trace_id, date, force)
Cron job endpoint

Firefly III has one endpoint for its various cron related tasks. Send a GET to this endpoint to run the cron. The cron requires the CLI token to be present. The cron job will fire for all users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cli_token** | **String** | The CLI token of any user in Firefly III, required to run the cron job. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**date** | Option<**String**> | A date formatted YYYY-MM-DD. This can be used to make the cron job pretend it's running on another day.  |  |
**force** | Option<**bool**> | Forces the cron job to fire, regardless of whether it has fired before. This may result in double transactions or weird budgets, so be careful.  |  |

### Return type

[**models::CronResult**](CronResult.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> models::UserSingle get_current_user(x_trace_id)
Currently authenticated user endpoint.

Returns the currently authenticated user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::UserSingle**](UserSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

