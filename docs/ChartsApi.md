# \ChartsApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_chart_account_overview**](ChartsApi.md#get_chart_account_overview) | **GET** /api/v1/chart/account/overview | Dashboard chart with asset account balance information.



## get_chart_account_overview

> Vec<crate::models::ChartDataSet> get_chart_account_overview(start, end)
Dashboard chart with asset account balance information.

This endpoint returns the data required to generate a chart with basic asset account balance information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |

### Return type

[**Vec<crate::models::ChartDataSet>**](ChartDataSet.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

