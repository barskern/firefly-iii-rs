# \AvailableBudgetsApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_available_budget**](AvailableBudgetsApi.md#get_available_budget) | **GET** /v1/available-budgets/{id} | Get a single available budget.
[**list_available_budgets**](AvailableBudgetsApi.md#list_available_budgets) | **GET** /v1/available-budgets | List all available budget amounts.



## get_available_budget

> models::AvailableBudgetSingle get_available_budget(id, x_trace_id)
Get a single available budget.

Get a single available budget, by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the available budget. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::AvailableBudgetSingle**](AvailableBudgetSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_budgets

> models::AvailableBudgetArray list_available_budgets(x_trace_id, limit, page, start, end)
List all available budget amounts.

Firefly III calculates the total amount of money budgeted in so-called \"available budgets\". This endpoint returns all of these amounts and the periods for which they are calculated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |

### Return type

[**models::AvailableBudgetArray**](AvailableBudgetArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

