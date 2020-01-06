# \CurrenciesApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**default_currency**](CurrenciesApi.md#default_currency) | **POST** /api/v1/currencies/{code}/default | Make currency default currency.
[**delete_currency**](CurrenciesApi.md#delete_currency) | **DELETE** /api/v1/currencies/{code} | Delete a currency.
[**disable_currency**](CurrenciesApi.md#disable_currency) | **POST** /api/v1/currencies/{code}/disable | Disable a currency.
[**enable_currency**](CurrenciesApi.md#enable_currency) | **POST** /api/v1/currencies/{code}/enable | Enable a single currency.
[**get_currency**](CurrenciesApi.md#get_currency) | **GET** /api/v1/currencies/{code} | Get a single currency.
[**list_account_by_currency**](CurrenciesApi.md#list_account_by_currency) | **GET** /api/v1/currencies/{code}/accounts | List all accounts with this currency.
[**list_available_budget_by_currency**](CurrenciesApi.md#list_available_budget_by_currency) | **GET** /api/v1/currencies/{code}/available_budgets | List all available budgets with this currency.
[**list_bill_by_currency**](CurrenciesApi.md#list_bill_by_currency) | **GET** /api/v1/currencies/{code}/bills | List all bills with this currency.
[**list_budget_limit_by_currency**](CurrenciesApi.md#list_budget_limit_by_currency) | **GET** /api/v1/currencies/{code}/budget_limits | List all budget limits with this currency
[**list_currency**](CurrenciesApi.md#list_currency) | **GET** /api/v1/currencies | List all currencies.
[**list_exchange_rate_by_currency**](CurrenciesApi.md#list_exchange_rate_by_currency) | **GET** /api/v1/currencies/{code}/cer | List all known exchange rates with (from or to) this currency.
[**list_recurrence_by_currency**](CurrenciesApi.md#list_recurrence_by_currency) | **GET** /api/v1/currencies/{code}/recurrences | List all recurring transactions with this currency.
[**list_rule_by_currency**](CurrenciesApi.md#list_rule_by_currency) | **GET** /api/v1/currencies/{code}/rules | List all rules with this currency.
[**list_transaction_by_currency**](CurrenciesApi.md#list_transaction_by_currency) | **GET** /api/v1/currencies/{code}/transactions | List all transactions with this currency.
[**store_currency**](CurrenciesApi.md#store_currency) | **POST** /api/v1/currencies | Store a new currency
[**update_currency**](CurrenciesApi.md#update_currency) | **PUT** /api/v1/currencies/{code} | Update existing currency.



## default_currency

> crate::models::CurrencySingle default_currency(code)
Make currency default currency.

Make this currency the default currency. If the currency is not enabled, it will be enabled as well.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |

### Return type

[**crate::models::CurrencySingle**](CurrencySingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_currency

> delete_currency(code)
Delete a currency.

Delete a currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_currency

> crate::models::CurrencySingle disable_currency(code)
Disable a currency.

Disable a currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **i32** | The currency code. | [required] |

### Return type

[**crate::models::CurrencySingle**](CurrencySingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_currency

> crate::models::CurrencySingle enable_currency(code)
Enable a single currency.

Enable a single currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |

### Return type

[**crate::models::CurrencySingle**](CurrencySingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_currency

> crate::models::CurrencySingle get_currency(code)
Get a single currency.

Get a single currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |

### Return type

[**crate::models::CurrencySingle**](CurrencySingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_account_by_currency

> crate::models::AccountArray list_account_by_currency(code, page, date, _type)
List all accounts with this currency.

List all accounts with this currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |
**date** | Option<**String**> | A date formatted YYYY-MM-DD. When added to the request, Firefly III will show the account's balance on that day.  |  |
**_type** | Option<[**crate::models::AccountTypeFilter**](.md)> | Optional filter on the account type(s) returned |  |

### Return type

[**crate::models::AccountArray**](AccountArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_budget_by_currency

> crate::models::AvailableBudgetArray list_available_budget_by_currency(code, page)
List all available budgets with this currency.

List all available budgets with this currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50 |  |

### Return type

[**crate::models::AvailableBudgetArray**](AvailableBudgetArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_bill_by_currency

> crate::models::BillArray list_bill_by_currency(code, page)
List all bills with this currency.

List all bills with this currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::BillArray**](BillArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_budget_limit_by_currency

> crate::models::BudgetLimitArray list_budget_limit_by_currency(code, page, start, end)
List all budget limits with this currency

List all budget limits with this currency

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |
**start** | Option<**String**> | Start date for the budget limit list. |  |
**end** | Option<**String**> | End date for the budget limit list. |  |

### Return type

[**crate::models::BudgetLimitArray**](BudgetLimitArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_currency

> crate::models::CurrencyArray list_currency(page)
List all currencies.

List all currencies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::CurrencyArray**](CurrencyArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_exchange_rate_by_currency

> crate::models::ExchangeRateArray list_exchange_rate_by_currency(code, page, date, start, end)
List all known exchange rates with (from or to) this currency.

List all known exchange rates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |
**date** | Option<**String**> | The date of which you want to know the exchange rate  |  |
**start** | Option<**String**> | Use this instead of the date parameter to search for a range of currency exchange values.  |  |
**end** | Option<**String**> | Use this instead of the date parameter to search for a range of currency exchange values.  |  |

### Return type

[**crate::models::ExchangeRateArray**](ExchangeRateArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recurrence_by_currency

> crate::models::RecurrenceArray list_recurrence_by_currency(code, page)
List all recurring transactions with this currency.

List all recurring transactions with this currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::RecurrenceArray**](RecurrenceArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rule_by_currency

> crate::models::RuleArray list_rule_by_currency(code, page)
List all rules with this currency.

List all rules with this currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |
**page** | Option<**i32**> | Page number. The default pagination per 50. |  |

### Return type

[**crate::models::RuleArray**](RuleArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transaction_by_currency

> crate::models::TransactionArray list_transaction_by_currency(code, page, start_date, end_date, _type)
List all transactions with this currency.

List all transactions with this currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is per 50. |  |
**start_date** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the list of transactions.  |  |
**end_date** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the list of transactions.  |  |
**_type** | Option<[**crate::models::TransactionTypeFilter**](.md)> | Optional filter on the transaction type(s) returned |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_currency

> crate::models::CurrencySingle store_currency(currency)
Store a new currency

Creates a new currency. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency** | [**Currency**](Currency.md) | JSON array or key=value pairs with the necessary currency information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::CurrencySingle**](CurrencySingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_currency

> crate::models::CurrencySingle update_currency(code, currency)
Update existing currency.

Update existing currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The currency code. | [required] |
**currency** | [**Currency**](Currency.md) | JSON array with updated currency information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::CurrencySingle**](CurrencySingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

