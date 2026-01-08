# \BudgetsApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_budget**](BudgetsApi.md#delete_budget) | **DELETE** /v1/budgets/{id} | Delete a budget.
[**delete_budget_limit**](BudgetsApi.md#delete_budget_limit) | **DELETE** /v1/budgets/{id}/limits/{limitId} | Delete a budget limit.
[**get_budget**](BudgetsApi.md#get_budget) | **GET** /v1/budgets/{id} | Get a single budget.
[**get_budget_limit**](BudgetsApi.md#get_budget_limit) | **GET** /v1/budgets/{id}/limits/{limitId} | Get single budget limit.
[**list_attachment_by_budget**](BudgetsApi.md#list_attachment_by_budget) | **GET** /v1/budgets/{id}/attachments | Lists all attachments of a budget.
[**list_budget**](BudgetsApi.md#list_budget) | **GET** /v1/budgets | List all budgets.
[**list_budget_limit**](BudgetsApi.md#list_budget_limit) | **GET** /v1/budget-limits | Get list of budget limits by date
[**list_budget_limit_by_budget**](BudgetsApi.md#list_budget_limit_by_budget) | **GET** /v1/budgets/{id}/limits | Get all limits for a budget.
[**list_transaction_by_budget**](BudgetsApi.md#list_transaction_by_budget) | **GET** /v1/budgets/{id}/transactions | All transactions to a budget.
[**list_transaction_by_budget_limit**](BudgetsApi.md#list_transaction_by_budget_limit) | **GET** /v1/budgets/{id}/limits/{limitId}/transactions | List all transactions by a budget limit ID.
[**list_transaction_without_budget**](BudgetsApi.md#list_transaction_without_budget) | **GET** /v1/budgets/transactions-without-budget | All transactions without a budget.
[**store_budget**](BudgetsApi.md#store_budget) | **POST** /v1/budgets | Store a new budget
[**store_budget_limit**](BudgetsApi.md#store_budget_limit) | **POST** /v1/budgets/{id}/limits | Store new budget limit.
[**update_budget**](BudgetsApi.md#update_budget) | **PUT** /v1/budgets/{id} | Update existing budget.
[**update_budget_limit**](BudgetsApi.md#update_budget_limit) | **PUT** /v1/budgets/{id}/limits/{limitId} | Update existing budget limit.



## delete_budget

> delete_budget(id, x_trace_id)
Delete a budget.

Delete a budget. Transactions will not be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the budget. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_budget_limit

> delete_budget_limit(id, limit_id, x_trace_id)
Delete a budget limit.

Delete a budget limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the budget. The budget limit MUST be associated to the budget ID. | [required] |
**limit_id** | **String** | The ID of the budget limit. The budget limit MUST be associated to the budget ID. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_budget

> models::BudgetSingle get_budget(id, x_trace_id, start, end)
Get a single budget.

Get a single budget. If the start date and end date are submitted as well, the \"spent\" array will be updated accordingly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the requested budget. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to get info on how much the user has spent.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to get info on how much the user has spent.  |  |

### Return type

[**models::BudgetSingle**](BudgetSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_budget_limit

> models::BudgetLimitSingle get_budget_limit(id, limit_id, x_trace_id)
Get single budget limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the budget. The budget limit MUST be associated to the budget ID. | [required] |
**limit_id** | **i32** | The ID of the budget limit. The budget limit MUST be associated to the budget ID. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::BudgetLimitSingle**](BudgetLimitSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment_by_budget

> models::AttachmentArray list_attachment_by_budget(id, x_trace_id, limit, page)
Lists all attachments of a budget.

Lists all attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the budget. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**models::AttachmentArray**](AttachmentArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_budget

> models::BudgetArray list_budget(x_trace_id, limit, page, start, end)
List all budgets.

List all the budgets the user has made. If the start date and end date are submitted as well, the \"spent\" array will be updated accordingly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to get info on how much the user has spent. You must submit both start and end.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to get info on how much the user has spent. You must submit both start and end.  |  |

### Return type

[**models::BudgetArray**](BudgetArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_budget_limit

> models::BudgetLimitArray list_budget_limit(start, end, x_trace_id)
Get list of budget limits by date

Get all budget limits for for this date range. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::BudgetLimitArray**](BudgetLimitArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_budget_limit_by_budget

> models::BudgetLimitArray list_budget_limit_by_budget(id, x_trace_id, start, end)
Get all limits for a budget.

Get all budget limits for this budget and the money spent, and money left. You can limit the list by submitting a date range as well. The \"spent\" array for each budget limit is NOT influenced by the start and end date of your query, but by the start and end date of the budget limit itself. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the requested budget. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |

### Return type

[**models::BudgetLimitArray**](BudgetLimitArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_budget

> models::TransactionArray list_transaction_by_budget(id, x_trace_id, limit, page, start, end, r#type)
All transactions to a budget.

Get all transactions linked to a budget, possibly limited by start and end

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the budget. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**r#type** | Option<[**TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned |  |

### Return type

[**models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_budget_limit

> models::TransactionArray list_transaction_by_budget_limit(id, limit_id, x_trace_id, limit, page, r#type)
List all transactions by a budget limit ID.

List all the transactions within one budget limit. The start and end date are dictated by the budget limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the budget. The budget limit MUST be associated to the budget ID. | [required] |
**limit_id** | **String** | The ID of the budget limit. The budget limit MUST be associated to the budget ID. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**r#type** | Option<[**TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned |  |

### Return type

[**models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_without_budget

> models::TransactionArray list_transaction_without_budget(x_trace_id, limit, page, start, end)
All transactions without a budget.

Get all transactions NOT linked to a budget, possibly limited by start and end

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**limit** | Option<**i32**> | Number of items per page. The default pagination is per 50 items. |  |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |

### Return type

[**models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_budget

> models::BudgetSingle store_budget(budget_store, x_trace_id)
Store a new budget

Creates a new budget. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_store** | [**BudgetStore**](BudgetStore.md) | JSON array or key=value pairs with the necessary budget information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::BudgetSingle**](BudgetSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_budget_limit

> models::BudgetLimitSingle store_budget_limit(id, budget_limit_store, x_trace_id)
Store new budget limit.

Store a new budget limit under this budget.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the budget. | [required] |
**budget_limit_store** | [**BudgetLimitStore**](BudgetLimitStore.md) | JSON array or key=value pairs with the necessary budget information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::BudgetLimitSingle**](BudgetLimitSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_budget

> models::BudgetSingle update_budget(id, budget_update, x_trace_id)
Update existing budget.

Update existing budget. This endpoint cannot be used to set budget amount limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the budget. | [required] |
**budget_update** | [**BudgetUpdate**](BudgetUpdate.md) | JSON array with updated budget information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::BudgetSingle**](BudgetSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_budget_limit

> models::BudgetLimitSingle update_budget_limit(id, limit_id, budget_limit_update, x_trace_id)
Update existing budget limit.

Update existing budget limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the budget. The budget limit MUST be associated to the budget ID. | [required] |
**limit_id** | **String** | The ID of the budget limit. The budget limit MUST be associated to the budget ID. | [required] |
**budget_limit_update** | [**BudgetLimitUpdate**](BudgetLimitUpdate.md) | JSON array with updated budget limit information. See the model for the exact specifications. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

[**models::BudgetLimitSingle**](BudgetLimitSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

