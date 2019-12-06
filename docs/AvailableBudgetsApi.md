# \AvailableBudgetsApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_available_budget**](AvailableBudgetsApi.md#delete_available_budget) | **DELETE** /api/v1/available_budgets/{id} | Delete an available budget.
[**get_available_budget**](AvailableBudgetsApi.md#get_available_budget) | **GET** /api/v1/available_budgets/{id} | Get a single available budget.
[**list_available_budget**](AvailableBudgetsApi.md#list_available_budget) | **GET** /api/v1/available_budgets | List all available budget amounts.
[**store_available_budget**](AvailableBudgetsApi.md#store_available_budget) | **POST** /api/v1/available_budgets | Store a new available budget
[**update_available_budget**](AvailableBudgetsApi.md#update_available_budget) | **PUT** /api/v1/available_budgets/{id} | Update existing available budget, to change for example the date range of the amount or the amount itself.



## delete_available_budget

> delete_available_budget(id)
Delete an available budget.

Delete an available budget. Not much more to say.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the available budget. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_available_budget

> crate::models::AvailableBudgetSingle get_available_budget(id)
Get a single available budget.

Get a single available budget, by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the available budget. | [required] |

### Return type

[**crate::models::AvailableBudgetSingle**](AvailableBudgetSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_budget

> crate::models::AvailableBudgetArray list_available_budget(page, start, end)
List all available budget amounts.

Firefly III allows users to set the amount that is available to be budgeted in so-called \"available budgets\". For example, the user could have 1200,- available to be divided during the coming month. This amount is used on the /budgets page. This endpoint returns all of these amounts and the periods for which they are set. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |

### Return type

[**crate::models::AvailableBudgetArray**](AvailableBudgetArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_available_budget

> crate::models::AvailableBudgetSingle store_available_budget(available_budget)
Store a new available budget

Creates a new available budget for a specified period. The data required can be submitted as a JSON body or as a list of parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**available_budget** | [**AvailableBudget**](AvailableBudget.md) | JSON array or key=value pairs with the necessary available budget information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::AvailableBudgetSingle**](AvailableBudgetSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_available_budget

> crate::models::AvailableBudgetSingle update_available_budget(id, available_budget)
Update existing available budget, to change for example the date range of the amount or the amount itself.

Update existing available budget.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the object.X | [required] |
**available_budget** | [**AvailableBudget**](AvailableBudget.md) | JSON array or form value with updated available budget information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::AvailableBudgetSingle**](AvailableBudgetSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

