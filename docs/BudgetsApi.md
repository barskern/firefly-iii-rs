# \BudgetsApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_budget**](BudgetsApi.md#delete_budget) | **DELETE** /api/v1/budgets/{id} | Delete a budget.
[**delete_budget_limit**](BudgetsApi.md#delete_budget_limit) | **DELETE** /api/v1/budgets/limits/{id} | Delete a budget limit.
[**get_budget**](BudgetsApi.md#get_budget) | **GET** /api/v1/budgets/{id} | Get a single budget.
[**get_budget_limit**](BudgetsApi.md#get_budget_limit) | **GET** /api/v1/budgets/limits/{id} | Get single budget limit.
[**list_attachment_by_budget**](BudgetsApi.md#list_attachment_by_budget) | **GET** /api/v1/budgets/{id}/attachments | Lists all attachments.
[**list_budget**](BudgetsApi.md#list_budget) | **GET** /api/v1/budgets | List all budgets.
[**list_budget_limit_by_budget**](BudgetsApi.md#list_budget_limit_by_budget) | **GET** /api/v1/budgets/{id}/limits | Get all limits
[**list_transaction_by_budget**](BudgetsApi.md#list_transaction_by_budget) | **GET** /api/v1/budgets/{id}/transactions | All transactions to a budget.
[**list_transaction_by_budget_limit**](BudgetsApi.md#list_transaction_by_budget_limit) | **GET** /api/v1/budgets/limits/{id}/transactions | List all transactions by a budget limit ID.
[**store_budget**](BudgetsApi.md#store_budget) | **POST** /api/v1/budgets | Store a new budget
[**store_budget_limit**](BudgetsApi.md#store_budget_limit) | **POST** /api/v1/budgets/{id}/limits | Store new budget limit.
[**update_budget**](BudgetsApi.md#update_budget) | **PUT** /api/v1/budgets/{id} | Update existing budget.
[**update_budget_limit**](BudgetsApi.md#update_budget_limit) | **PUT** /api/v1/budgets/limits/{id} | Update existing budget limit.



## delete_budget

> delete_budget(id)
Delete a budget.

Delete a budget. Transactions will not be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the budget. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_budget_limit

> delete_budget_limit(id)
Delete a budget limit.

Delete a budget limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the requested budget limit. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_budget

> crate::models::BudgetSingle get_budget(id, start_date, end_date)
Get a single budget.

Get a single budget. If the start date and end date are submitted as well, the \"spent\" array will be updated accordingly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the requested budget. | [required] |
**start_date** | Option<**String**> | A date formatted YYYY-MM-DD, to get info on how much the user has spent.  |  |
**end_date** | Option<**String**> | A date formatted YYYY-MM-DD, to get info on how much the user has spent.  |  |

### Return type

[**crate::models::BudgetSingle**](BudgetSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_budget_limit

> crate::models::BudgetLimitSingle get_budget_limit(id)
Get single budget limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the requested budget limit. | [required] |

### Return type

[**crate::models::BudgetLimitSingle**](BudgetLimitSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment_by_budget

> crate::models::AttachmentArray list_attachment_by_budget(id, page)
Lists all attachments.

Lists all attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the budget. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::AttachmentArray**](AttachmentArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_budget

> crate::models::BudgetArray list_budget(page, start, end)
List all budgets.

List all the budgets the user has made. If the start date and end date are submitted as well, the \"spent\" array will be updated accordingly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to get info on how much the user has spent. You must submit both start and end.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to get info on how much the user has spent. You must submit both start and end.  |  |

### Return type

[**crate::models::BudgetArray**](BudgetArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_budget_limit_by_budget

> crate::models::BudgetLimitArray list_budget_limit_by_budget(id, start, end)
Get all limits

Get all budget limits for this budget and the money spent, and money left. You can limit the list by submitting a date range as well. The \"spent\" array for each budget limit is NOT influenced by the start and end date of your query, but by the start and end date of the budget limit itself. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the requested budget. | [required] |
**start** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |

### Return type

[**crate::models::BudgetLimitArray**](BudgetLimitArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_budget

> crate::models::TransactionArray list_transaction_by_budget(id, limit, page, start, end, _type)
All transactions to a budget.

Get all transactions linked to a budget, possibly limited by start and end

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the budget. | [required] |
**limit** | Option<**i32**> | Limits the number of results on one page. |  |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD.  |  |
**_type** | Option<[**crate::models::TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_budget_limit

> crate::models::TransactionArray list_transaction_by_budget_limit(id, page, _type)
List all transactions by a budget limit ID.

List all the transactions within one budget limit. The start and end date are dictated by the budget limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the requested budget limit. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |
**_type** | Option<[**crate::models::TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_budget

> crate::models::BudgetSingle store_budget(budget)
Store a new budget

Creates a new budget. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget** | [**Budget**](Budget.md) | JSON array or key=value pairs with the necessary budget information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::BudgetSingle**](BudgetSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_budget_limit

> crate::models::BudgetLimitSingle store_budget_limit(id, budget_limit)
Store new budget limit.

Store a new budget limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the budget. | [required] |
**budget_limit** | [**BudgetLimit**](BudgetLimit.md) | JSON array or key=value pairs with the necessary budget information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::BudgetLimitSingle**](BudgetLimitSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_budget

> crate::models::BudgetSingle update_budget(id, budget)
Update existing budget.

Update existing budget. This endpoint cannot be used to set budget amount limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the budget. | [required] |
**budget** | [**Budget**](Budget.md) | JSON array with updated budget information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::BudgetSingle**](BudgetSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_budget_limit

> crate::models::BudgetLimitSingle update_budget_limit(id, budget_limit)
Update existing budget limit.

Update existing budget limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the requested budget limit. The budget limit MUST be associated to the budget ID. | [required] |
**budget_limit** | [**BudgetLimit**](BudgetLimit.md) | JSON array with updated budget limit information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::BudgetLimitSingle**](BudgetLimitSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

