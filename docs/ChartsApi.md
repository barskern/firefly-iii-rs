# \ChartsApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_chart_ab_overview**](ChartsApi.md#get_chart_ab_overview) | **GET** /api/v1/chart/ab/overview/{id} | Dashboard chart with an overview of the available budget.
[**get_chart_account_expense**](ChartsApi.md#get_chart_account_expense) | **GET** /api/v1/chart/account/expense | Dashboard chart with expense account balance information.
[**get_chart_account_overview**](ChartsApi.md#get_chart_account_overview) | **GET** /api/v1/chart/account/overview | Dashboard chart with asset account balance information.
[**get_chart_account_revenue**](ChartsApi.md#get_chart_account_revenue) | **GET** /api/v1/chart/account/revenue | Dashboard chart with revenue account balance information.
[**get_chart_category_overview**](ChartsApi.md#get_chart_category_overview) | **GET** /api/v1/chart/category/overview | Dashboard chart with an overview of the users categories.



## get_chart_ab_overview

> Vec<crate::models::ChartDataSet> get_chart_ab_overview(id, start, end)
Dashboard chart with an overview of the available budget.

This endpoint returns the data required to generate a pie chart for the available budget. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the available budget. | [required] |
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


## get_chart_account_expense

> Vec<crate::models::ChartDataSet> get_chart_account_expense(start, end)
Dashboard chart with expense account balance information.

This endpoint returns the data required to generate a chart that shows the user how much they've spent on their expense accounts. 

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


## get_chart_account_revenue

> Vec<crate::models::ChartDataSet> get_chart_account_revenue(start, end)
Dashboard chart with revenue account balance information.

This endpoint returns the data required to generate a chart that shows the user how much they've earned from their revenue accounts. 

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


## get_chart_category_overview

> Vec<crate::models::ChartDataSet> get_chart_category_overview(start, end)
Dashboard chart with an overview of the users categories.

This endpoint returns the data required to generate a bar chart for the expenses and incomes on the users categories. 

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

