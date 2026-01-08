# \ChartsApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_chart_account_overview**](ChartsApi.md#get_chart_account_overview) | **GET** /v1/chart/account/overview | Dashboard chart with asset account balance information.
[**get_chart_balance**](ChartsApi.md#get_chart_balance) | **GET** /v1/chart/balance/balance | Dashboard chart with balance information.
[**get_chart_budget_overview**](ChartsApi.md#get_chart_budget_overview) | **GET** /v1/chart/budget/overview | Dashboard chart with budget information.
[**get_chart_category_overview**](ChartsApi.md#get_chart_category_overview) | **GET** /v1/chart/category/overview | Dashboard chart with category information.



## get_chart_account_overview

> Vec<models::ChartDataSet> get_chart_account_overview(start, end, x_trace_id, period, preselected)
Dashboard chart with asset account balance information.

This endpoint returns the data required to generate a chart with basic asset account balance information. This is used on the dashboard. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**period** | Option<**String**> | Optional period to group the data by. If not provided, it will default to '1M' or whatever is deemed relevant for the range provided.  If you want to know which periods are available, see the enums or get the configuration value: `GET /api/v1/configuration/firefly.valid_view_ranges`  |  |
**preselected** | Option<**String**> | Optional set of preselected accounts to limit the chart to. This may be easier than submitting all asset accounts manually, for example. If you want to know which selection are available, see the enums here or get the configuration value: `GET /api/v1/configuration/firefly.preselected_accounts`  - `empty`: do not do a pre-selection - `all`: select all asset and all liability accounts - `assets`: select all asset accounts - `liabilities`: select all liability accounts  If no accounts are found, the user's \"frontpage accounts\" preference will be used. If that is empty, all asset accounts will be used.  |  |

### Return type

[**Vec<models::ChartDataSet>**](ChartDataSet.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chart_balance

> Vec<models::ChartDataSet> get_chart_balance(start, end, x_trace_id, period, preselected, accounts_left_square_bracket_right_square_bracket)
Dashboard chart with balance information.

This endpoint returns the data required to generate a chart with balance information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**period** | Option<**String**> | Optional period to group the data by. If not provided, it will default to '1M' or whatever is deemed relevant for the range provided.  If you want to know which periods are available, see the enums or get the configuration value: `GET /api/v1/configuration/firefly.valid_view_ranges`  |  |
**preselected** | Option<**String**> | Optional set of preselected accounts to limit the chart to. This may be easier than submitting all asset accounts manually, for example. If you want to know which selection are available, see the enums here or get the configuration value: `GET /api/v1/configuration/firefly.preselected_accounts`  - `empty`: do not do a pre-selection - `all`: select all asset and all liability accounts - `assets`: select all asset accounts - `liabilities`: select all liability accounts  If no accounts are found, the user's \"frontpage accounts\" preference will be used. If that is empty, all asset accounts will be used.  |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | Limit the chart to these asset accounts or liabilities. Only asset accounts and liabilities will be accepted. Other types will be silently dropped.  This list of accounts will be OVERRULED by the `preselected` parameter.  |  |

### Return type

[**Vec<models::ChartDataSet>**](ChartDataSet.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chart_budget_overview

> Vec<models::ChartDataSet> get_chart_budget_overview(start, end, x_trace_id)
Dashboard chart with budget information.

This endpoint returns the data required to generate a chart with basic budget information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**Vec<models::ChartDataSet>**](ChartDataSet.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chart_category_overview

> Vec<models::ChartDataSet> get_chart_category_overview(start, end, x_trace_id)
Dashboard chart with category information.

This endpoint returns the data required to generate a chart with basic category information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**Vec<models::ChartDataSet>**](ChartDataSet.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

