# \InsightApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**insight_expense_asset**](InsightApi.md#insight_expense_asset) | **GET** /v1/insight/expense/asset | Insight into expenses, grouped by asset account.
[**insight_expense_bill**](InsightApi.md#insight_expense_bill) | **GET** /v1/insight/expense/bill | Insight into expenses, grouped by bill.
[**insight_expense_budget**](InsightApi.md#insight_expense_budget) | **GET** /v1/insight/expense/budget | Insight into expenses, grouped by budget.
[**insight_expense_category**](InsightApi.md#insight_expense_category) | **GET** /v1/insight/expense/category | Insight into expenses, grouped by category.
[**insight_expense_expense**](InsightApi.md#insight_expense_expense) | **GET** /v1/insight/expense/expense | Insight into expenses, grouped by expense account.
[**insight_expense_no_bill**](InsightApi.md#insight_expense_no_bill) | **GET** /v1/insight/expense/no-bill | Insight into expenses, without bill.
[**insight_expense_no_budget**](InsightApi.md#insight_expense_no_budget) | **GET** /v1/insight/expense/no-budget | Insight into expenses, without budget.
[**insight_expense_no_category**](InsightApi.md#insight_expense_no_category) | **GET** /v1/insight/expense/no-category | Insight into expenses, without category.
[**insight_expense_no_tag**](InsightApi.md#insight_expense_no_tag) | **GET** /v1/insight/expense/no-tag | Insight into expenses, without tag.
[**insight_expense_tag**](InsightApi.md#insight_expense_tag) | **GET** /v1/insight/expense/tag | Insight into expenses, grouped by tag.
[**insight_expense_total**](InsightApi.md#insight_expense_total) | **GET** /v1/insight/expense/total | Insight into total expenses.
[**insight_income_asset**](InsightApi.md#insight_income_asset) | **GET** /v1/insight/income/asset | Insight into income, grouped by asset account.
[**insight_income_category**](InsightApi.md#insight_income_category) | **GET** /v1/insight/income/category | Insight into income, grouped by category.
[**insight_income_no_category**](InsightApi.md#insight_income_no_category) | **GET** /v1/insight/income/no-category | Insight into income, without category.
[**insight_income_no_tag**](InsightApi.md#insight_income_no_tag) | **GET** /v1/insight/income/no-tag | Insight into income, without tag.
[**insight_income_revenue**](InsightApi.md#insight_income_revenue) | **GET** /v1/insight/income/revenue | Insight into income, grouped by revenue account.
[**insight_income_tag**](InsightApi.md#insight_income_tag) | **GET** /v1/insight/income/tag | Insight into income, grouped by tag.
[**insight_income_total**](InsightApi.md#insight_income_total) | **GET** /v1/insight/income/total | Insight into total income.
[**insight_transfer_category**](InsightApi.md#insight_transfer_category) | **GET** /v1/insight/transfer/category | Insight into transfers, grouped by category.
[**insight_transfer_no_category**](InsightApi.md#insight_transfer_no_category) | **GET** /v1/insight/transfer/no-category | Insight into transfers, without category.
[**insight_transfer_no_tag**](InsightApi.md#insight_transfer_no_tag) | **GET** /v1/insight/transfer/no-tag | Insight into expenses, without tag.
[**insight_transfer_tag**](InsightApi.md#insight_transfer_tag) | **GET** /v1/insight/transfer/tag | Insight into transfers, grouped by tag.
[**insight_transfer_total**](InsightApi.md#insight_transfer_total) | **GET** /v1/insight/transfer/total | Insight into total transfers.
[**insight_transfers**](InsightApi.md#insight_transfers) | **GET** /v1/insight/transfer/asset | Insight into transfers, grouped by account.



## insight_expense_asset

> Vec<models::InsightGroupEntry> insight_expense_asset(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, grouped by asset account.

This endpoint gives a summary of the expenses made by the user, grouped by asset account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_bill

> Vec<models::InsightGroupEntry> insight_expense_bill(start, end, x_trace_id, bills_left_square_bracket_right_square_bracket, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, grouped by bill.

This endpoint gives a summary of the expenses made by the user, grouped by (any) bill. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**bills_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The bills to be included in the results.  |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_budget

> Vec<models::InsightGroupEntry> insight_expense_budget(start, end, x_trace_id, budgets_left_square_bracket_right_square_bracket, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, grouped by budget.

This endpoint gives a summary of the expenses made by the user, grouped by (any) budget. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**budgets_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The budgets to be included in the results.  |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_category

> Vec<models::InsightGroupEntry> insight_expense_category(start, end, x_trace_id, categories_left_square_bracket_right_square_bracket, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, grouped by category.

This endpoint gives a summary of the expenses made by the user, grouped by (any) category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**categories_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The categories to be included in the results.  |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_expense

> Vec<models::InsightGroupEntry> insight_expense_expense(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, grouped by expense account.

This endpoint gives a summary of the expenses made by the user, grouped by expense account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you add the accounts ID's of expense accounts, only those accounts are included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. You can combine both asset / liability and expense account ID's. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_no_bill

> Vec<models::InsightTotalEntry> insight_expense_no_bill(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, without bill.

This endpoint gives a summary of the expenses made by the user, including only expenses with no bill. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_no_budget

> Vec<models::InsightTotalEntry> insight_expense_no_budget(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, without budget.

This endpoint gives a summary of the expenses made by the user, including only expenses with no budget. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_no_category

> Vec<models::InsightTotalEntry> insight_expense_no_category(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, without category.

This endpoint gives a summary of the expenses made by the user, including only expenses with no category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_no_tag

> Vec<models::InsightTotalEntry> insight_expense_no_tag(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, without tag.

This endpoint gives a summary of the expenses made by the user, including only expenses with no tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_tag

> Vec<models::InsightGroupEntry> insight_expense_tag(start, end, x_trace_id, tags_left_square_bracket_right_square_bracket, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, grouped by tag.

This endpoint gives a summary of the expenses made by the user, grouped by (any) tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**tags_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The tags to be included in the results.  |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_total

> Vec<models::InsightTotalEntry> insight_expense_total(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into total expenses.

This endpoint gives a sum of the total expenses made by the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_asset

> Vec<models::InsightGroupEntry> insight_income_asset(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into income, grouped by asset account.

This endpoint gives a summary of the income received by the user, grouped by asset account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_category

> Vec<models::InsightGroupEntry> insight_income_category(start, end, x_trace_id, categories_left_square_bracket_right_square_bracket, accounts_left_square_bracket_right_square_bracket)
Insight into income, grouped by category.

This endpoint gives a summary of the income received by the user, grouped by (any) category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**categories_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The categories to be included in the results.  |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_no_category

> Vec<models::InsightTotalEntry> insight_income_no_category(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into income, without category.

This endpoint gives a summary of the income received by the user, including only income with no category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_no_tag

> Vec<models::InsightTotalEntry> insight_income_no_tag(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into income, without tag.

This endpoint gives a summary of the income received by the user, including only income with no tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_revenue

> Vec<models::InsightGroupEntry> insight_income_revenue(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into income, grouped by revenue account.

This endpoint gives a summary of the income received by the user, grouped by revenue account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you add the accounts ID's of revenue accounts, only those accounts are included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. You can combine both asset / liability and deposit account ID's. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_tag

> Vec<models::InsightGroupEntry> insight_income_tag(start, end, x_trace_id, tags_left_square_bracket_right_square_bracket, accounts_left_square_bracket_right_square_bracket)
Insight into income, grouped by tag.

This endpoint gives a summary of the income received by the user, grouped by (any) tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**tags_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The tags to be included in the results.  |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_total

> Vec<models::InsightTotalEntry> insight_income_total(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into total income.

This endpoint gives a sum of the total income received by the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfer_category

> Vec<models::InsightGroupEntry> insight_transfer_category(start, end, x_trace_id, categories_left_square_bracket_right_square_bracket, accounts_left_square_bracket_right_square_bracket)
Insight into transfers, grouped by category.

This endpoint gives a summary of the transfers made by the user, grouped by (any) category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**categories_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The categories to be included in the results.  |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers between those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfer_no_category

> Vec<models::InsightTotalEntry> insight_transfer_no_category(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into transfers, without category.

This endpoint gives a summary of the transfers made by the user, including only transfers with no category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers between those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfer_no_tag

> Vec<models::InsightTotalEntry> insight_transfer_no_tag(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into expenses, without tag.

This endpoint gives a summary of the transfers made by the user, including only transfers with no tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfer_tag

> Vec<models::InsightGroupEntry> insight_transfer_tag(start, end, x_trace_id, tags_left_square_bracket_right_square_bracket, accounts_left_square_bracket_right_square_bracket)
Insight into transfers, grouped by tag.

This endpoint gives a summary of the transfers created by the user, grouped by (any) tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**tags_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The tags to be included in the results.  |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers between those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfer_total

> Vec<models::InsightTotalEntry> insight_transfer_total(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into total transfers.

This endpoint gives a sum of the total amount transfers made by the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers between those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfers

> Vec<models::InsightTransferEntry> insight_transfers(start, end, x_trace_id, accounts_left_square_bracket_right_square_bracket)
Insight into transfers, grouped by account.

This endpoint gives a summary of the transfers made by the user, grouped by asset account or lability. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts_left_square_bracket_right_square_bracket** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers between those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<models::InsightTransferEntry>**](InsightTransferEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

