# \InsightApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**insight_expense_asset**](InsightApi.md#insight_expense_asset) | **GET** /api/v1/insight/expense/asset | Insight into expenses, grouped by asset account.
[**insight_expense_bill**](InsightApi.md#insight_expense_bill) | **GET** /api/v1/insight/expense/bill | Insight into expenses, grouped by bill.
[**insight_expense_budget**](InsightApi.md#insight_expense_budget) | **GET** /api/v1/insight/expense/budget | Insight into expenses, grouped by budget.
[**insight_expense_category**](InsightApi.md#insight_expense_category) | **GET** /api/v1/insight/expense/category | Insight into expenses, grouped by category.
[**insight_expense_expense**](InsightApi.md#insight_expense_expense) | **GET** /api/v1/insight/expense/expense | Insight into expenses, grouped by expense account.
[**insight_expense_no_bill**](InsightApi.md#insight_expense_no_bill) | **GET** /api/v1/insight/expense/no-bill | Insight into expenses, without bill.
[**insight_expense_no_budget**](InsightApi.md#insight_expense_no_budget) | **GET** /api/v1/insight/expense/no-budget | Insight into expenses, without budget.
[**insight_expense_no_category**](InsightApi.md#insight_expense_no_category) | **GET** /api/v1/insight/expense/no-category | Insight into expenses, without category.
[**insight_expense_no_tag**](InsightApi.md#insight_expense_no_tag) | **GET** /api/v1/insight/expense/no-tag | Insight into expenses, without tag.
[**insight_expense_tag**](InsightApi.md#insight_expense_tag) | **GET** /api/v1/insight/expense/tag | Insight into expenses, grouped by tag.
[**insight_expense_total**](InsightApi.md#insight_expense_total) | **GET** /api/v1/insight/expense/total | Insight into total expenses.
[**insight_income_asset**](InsightApi.md#insight_income_asset) | **GET** /api/v1/insight/income/asset | Insight into income, grouped by asset account.
[**insight_income_category**](InsightApi.md#insight_income_category) | **GET** /api/v1/insight/income/category | Insight into income, grouped by category.
[**insight_income_no_category**](InsightApi.md#insight_income_no_category) | **GET** /api/v1/insight/income/no-category | Insight into income, without category.
[**insight_income_no_tag**](InsightApi.md#insight_income_no_tag) | **GET** /api/v1/insight/income/no-tag | Insight into income, without tag.
[**insight_income_revenue**](InsightApi.md#insight_income_revenue) | **GET** /api/v1/insight/income/revenue | Insight into income, grouped by revenue account.
[**insight_income_tag**](InsightApi.md#insight_income_tag) | **GET** /api/v1/insight/income/tag | Insight into income, grouped by tag.
[**insight_income_total**](InsightApi.md#insight_income_total) | **GET** /api/v1/insight/income/total | Insight into total income.
[**insight_transfer_category**](InsightApi.md#insight_transfer_category) | **GET** /api/v1/insight/transfer/category | Insight into transfers, grouped by category.
[**insight_transfer_no_category**](InsightApi.md#insight_transfer_no_category) | **GET** /api/v1/insight/transfer/no-category | Insight into transfers, without category.
[**insight_transfer_no_tag**](InsightApi.md#insight_transfer_no_tag) | **GET** /api/v1/insight/transfer/no-tag | Insight into expenses, without tag.
[**insight_transfer_tag**](InsightApi.md#insight_transfer_tag) | **GET** /api/v1/insight/transfer/tag | Insight into transfers, grouped by tag.
[**insight_transfer_total**](InsightApi.md#insight_transfer_total) | **GET** /api/v1/insight/transfer/total | Insight into total transfers.
[**insight_transfers**](InsightApi.md#insight_transfers) | **GET** /api/v1/insight/transfer/asset | Insight into transfers, grouped by account.



## insight_expense_asset

> Vec<crate::models::InsightGroupEntry> insight_expense_asset(start, end, accounts)
Insight into expenses, grouped by asset account.

This endpoint gives a summary of the expenses made by the user, grouped by asset account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_bill

> Vec<crate::models::InsightGroupEntry> insight_expense_bill(start, end, bills, accounts)
Insight into expenses, grouped by bill.

This endpoint gives a summary of the expenses made by the user, grouped by (any) bill. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**bills** | Option<[**Vec<i64>**](i64.md)> | The bills to be included in the results.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_budget

> Vec<crate::models::InsightGroupEntry> insight_expense_budget(start, end, budgets, accounts)
Insight into expenses, grouped by budget.

This endpoint gives a summary of the expenses made by the user, grouped by (any) budget. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**budgets** | Option<[**Vec<i64>**](i64.md)> | The budgets to be included in the results.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_category

> Vec<crate::models::InsightGroupEntry> insight_expense_category(start, end, categories, accounts)
Insight into expenses, grouped by category.

This endpoint gives a summary of the expenses made by the user, grouped by (any) category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**categories** | Option<[**Vec<i64>**](i64.md)> | The categories to be included in the results.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_expense

> Vec<crate::models::InsightGroupEntry> insight_expense_expense(start, end, accounts)
Insight into expenses, grouped by expense account.

This endpoint gives a summary of the expenses made by the user, grouped by expense account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you add the accounts ID's of expense accounts, only those accounts are included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. You can combine both asset / liability and expense account ID's. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_no_bill

> Vec<crate::models::InsightTotalEntry> insight_expense_no_bill(start, end, accounts)
Insight into expenses, without bill.

This endpoint gives a summary of the expenses made by the user, including only expenses with no bill. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_no_budget

> Vec<crate::models::InsightTotalEntry> insight_expense_no_budget(start, end, accounts)
Insight into expenses, without budget.

This endpoint gives a summary of the expenses made by the user, including only expenses with no budget. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_no_category

> Vec<crate::models::InsightTotalEntry> insight_expense_no_category(start, end, accounts)
Insight into expenses, without category.

This endpoint gives a summary of the expenses made by the user, including only expenses with no category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_no_tag

> Vec<crate::models::InsightTotalEntry> insight_expense_no_tag(start, end, accounts)
Insight into expenses, without tag.

This endpoint gives a summary of the expenses made by the user, including only expenses with no tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_tag

> Vec<crate::models::InsightGroupEntry> insight_expense_tag(start, end, tags, accounts)
Insight into expenses, grouped by tag.

This endpoint gives a summary of the expenses made by the user, grouped by (any) tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**tags** | Option<[**Vec<i64>**](i64.md)> | The tags to be included in the results.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_expense_total

> Vec<crate::models::InsightTotalEntry> insight_expense_total(start, end, accounts)
Insight into total expenses.

This endpoint gives a sum of the total expenses made by the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only withdrawals from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_asset

> Vec<crate::models::InsightGroupEntry> insight_income_asset(start, end, accounts)
Insight into income, grouped by asset account.

This endpoint gives a summary of the income received by the user, grouped by asset account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_category

> Vec<crate::models::InsightGroupEntry> insight_income_category(start, end, categories, accounts)
Insight into income, grouped by category.

This endpoint gives a summary of the income received by the user, grouped by (any) category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**categories** | Option<[**Vec<i64>**](i64.md)> | The categories to be included in the results.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_no_category

> Vec<crate::models::InsightTotalEntry> insight_income_no_category(start, end, accounts)
Insight into income, without category.

This endpoint gives a summary of the income received by the user, including only income with no category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_no_tag

> Vec<crate::models::InsightTotalEntry> insight_income_no_tag(start, end, accounts)
Insight into income, without tag.

This endpoint gives a summary of the income received by the user, including only income with no tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_revenue

> Vec<crate::models::InsightGroupEntry> insight_income_revenue(start, end, accounts)
Insight into income, grouped by revenue account.

This endpoint gives a summary of the income received by the user, grouped by revenue account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you add the accounts ID's of revenue accounts, only those accounts are included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. You can combine both asset / liability and deposit account ID's. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_tag

> Vec<crate::models::InsightGroupEntry> insight_income_tag(start, end, tags, accounts)
Insight into income, grouped by tag.

This endpoint gives a summary of the income received by the user, grouped by (any) tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**tags** | Option<[**Vec<i64>**](i64.md)> | The tags to be included in the results.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_income_total

> Vec<crate::models::InsightTotalEntry> insight_income_total(start, end, accounts)
Insight into total income.

This endpoint gives a sum of the total income received by the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only deposits to those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfer_category

> Vec<crate::models::InsightGroupEntry> insight_transfer_category(start, end, categories, accounts)
Insight into transfers, grouped by category.

This endpoint gives a summary of the transfers made by the user, grouped by (any) category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**categories** | Option<[**Vec<i64>**](i64.md)> | The categories to be included in the results.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers between those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfer_no_category

> Vec<crate::models::InsightTotalEntry> insight_transfer_no_category(start, end, accounts)
Insight into transfers, without category.

This endpoint gives a summary of the transfers made by the user, including only transfers with no category. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers between those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfer_no_tag

> Vec<crate::models::InsightTotalEntry> insight_transfer_no_tag(start, end, accounts)
Insight into expenses, without tag.

This endpoint gives a summary of the transfers made by the user, including only transfers with no tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers from those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfer_tag

> Vec<crate::models::InsightGroupEntry> insight_transfer_tag(start, end, tags, accounts)
Insight into transfers, grouped by tag.

This endpoint gives a summary of the transfers created by the user, grouped by (any) tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**tags** | Option<[**Vec<i64>**](i64.md)> | The tags to be included in the results.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers between those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightGroupEntry>**](InsightGroupEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfer_total

> Vec<crate::models::InsightTotalEntry> insight_transfer_total(start, end, accounts)
Insight into total transfers.

This endpoint gives a sum of the total amount transfers made by the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers between those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTotalEntry>**](InsightTotalEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_transfers

> Vec<crate::models::InsightTransferEntry> insight_transfers(start, end, accounts)
Insight into transfers, grouped by account.

This endpoint gives a summary of the transfers made by the user, grouped by asset account or lability. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<[**Vec<i64>**](i64.md)> | The accounts to be included in the results. If you include ID's of asset accounts or liabilities, only transfers between those asset accounts / liabilities will be included. Other account ID's will be ignored.  |  |

### Return type

[**Vec<crate::models::InsightTransferEntry>**](InsightTransferEntry.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

