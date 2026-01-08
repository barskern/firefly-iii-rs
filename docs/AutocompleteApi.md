# \AutocompleteApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_accounts_ac**](AutocompleteApi.md#get_accounts_ac) | **GET** /v1/autocomplete/accounts | Returns all accounts of the user returned in a basic auto-complete array.
[**get_bills_ac**](AutocompleteApi.md#get_bills_ac) | **GET** /v1/autocomplete/bills | Returns all bills of the user returned in a basic auto-complete array.
[**get_budgets_ac**](AutocompleteApi.md#get_budgets_ac) | **GET** /v1/autocomplete/budgets | Returns all budgets of the user returned in a basic auto-complete array.
[**get_categories_ac**](AutocompleteApi.md#get_categories_ac) | **GET** /v1/autocomplete/categories | Returns all categories of the user returned in a basic auto-complete array.
[**get_currencies_ac**](AutocompleteApi.md#get_currencies_ac) | **GET** /v1/autocomplete/currencies | Returns all currencies of the user returned in a basic auto-complete array.
[**get_currencies_code_ac**](AutocompleteApi.md#get_currencies_code_ac) | **GET** /v1/autocomplete/currencies-with-code | Returns all currencies of the user returned in a basic auto-complete array. This endpoint is DEPRECATED and I suggest you DO NOT use it.
[**get_object_groups_ac**](AutocompleteApi.md#get_object_groups_ac) | **GET** /v1/autocomplete/object-groups | Returns all object groups of the user returned in a basic auto-complete array.
[**get_piggies_ac**](AutocompleteApi.md#get_piggies_ac) | **GET** /v1/autocomplete/piggy-banks | Returns all piggy banks of the user returned in a basic auto-complete array.
[**get_piggies_balance_ac**](AutocompleteApi.md#get_piggies_balance_ac) | **GET** /v1/autocomplete/piggy-banks-with-balance | Returns all piggy banks of the user returned in a basic auto-complete array.
[**get_recurring_ac**](AutocompleteApi.md#get_recurring_ac) | **GET** /v1/autocomplete/recurring | Returns all recurring transactions of the user returned in a basic auto-complete array.
[**get_rule_groups_ac**](AutocompleteApi.md#get_rule_groups_ac) | **GET** /v1/autocomplete/rule-groups | Returns all rule groups of the user returned in a basic auto-complete array.
[**get_rules_ac**](AutocompleteApi.md#get_rules_ac) | **GET** /v1/autocomplete/rules | Returns all rules of the user returned in a basic auto-complete array.
[**get_subscriptions_ac**](AutocompleteApi.md#get_subscriptions_ac) | **GET** /v1/autocomplete/subscriptions | Returns all subscriptions of the user returned in a basic auto-complete array.
[**get_tag_ac**](AutocompleteApi.md#get_tag_ac) | **GET** /v1/autocomplete/tags | Returns all tags of the user returned in a basic auto-complete array.
[**get_transaction_types_ac**](AutocompleteApi.md#get_transaction_types_ac) | **GET** /v1/autocomplete/transaction-types | Returns all transaction types returned in a basic auto-complete array. English only.
[**get_transactions_ac**](AutocompleteApi.md#get_transactions_ac) | **GET** /v1/autocomplete/transactions | Returns all transaction descriptions of the user returned in a basic auto-complete array.
[**get_transactions_idac**](AutocompleteApi.md#get_transactions_idac) | **GET** /v1/autocomplete/transactions-with-id | Returns all transactions, complemented with their ID, of the user returned in a basic auto-complete array. This endpoint is DEPRECATED and I suggest you DO NOT use it.



## get_accounts_ac

> Vec<models::AutocompleteAccount> get_accounts_ac(x_trace_id, query, limit, date, types)
Returns all accounts of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |
**date** | Option<**String**> | If the account is an asset account or a liability, the autocomplete will also return the balance of the account on this date. |  |
**types** | Option<[**Vec<models::AccountTypeFilter>**](models::AccountTypeFilter.md)> | Optional filter on the account type(s) used in the autocomplete. |  |

### Return type

[**Vec<models::AutocompleteAccount>**](AutocompleteAccount.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bills_ac

> Vec<models::AutocompleteBill> get_bills_ac(x_trace_id, query, limit)
Returns all bills of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteBill>**](AutocompleteBill.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_budgets_ac

> Vec<models::AutocompleteBudget> get_budgets_ac(x_trace_id, query, limit)
Returns all budgets of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteBudget>**](AutocompleteBudget.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_categories_ac

> Vec<models::AutocompleteCategory> get_categories_ac(x_trace_id, query, limit)
Returns all categories of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteCategory>**](AutocompleteCategory.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_currencies_ac

> Vec<models::AutocompleteCurrency> get_currencies_ac(x_trace_id, query, limit)
Returns all currencies of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteCurrency>**](AutocompleteCurrency.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_currencies_code_ac

> Vec<models::AutocompleteCurrencyCode> get_currencies_code_ac(x_trace_id, query, limit)
Returns all currencies of the user returned in a basic auto-complete array. This endpoint is DEPRECATED and I suggest you DO NOT use it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteCurrencyCode>**](AutocompleteCurrencyCode.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_groups_ac

> Vec<models::AutocompleteObjectGroup> get_object_groups_ac(x_trace_id, query, limit)
Returns all object groups of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteObjectGroup>**](AutocompleteObjectGroup.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_piggies_ac

> Vec<models::AutocompletePiggy> get_piggies_ac(x_trace_id, query, limit)
Returns all piggy banks of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompletePiggy>**](AutocompletePiggy.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_piggies_balance_ac

> Vec<models::AutocompletePiggyBalance> get_piggies_balance_ac(x_trace_id, query, limit)
Returns all piggy banks of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompletePiggyBalance>**](AutocompletePiggyBalance.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recurring_ac

> Vec<models::AutocompleteRecurrence> get_recurring_ac(x_trace_id, query, limit)
Returns all recurring transactions of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteRecurrence>**](AutocompleteRecurrence.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_groups_ac

> Vec<models::AutocompleteRuleGroup> get_rule_groups_ac(x_trace_id, query, limit)
Returns all rule groups of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteRuleGroup>**](AutocompleteRuleGroup.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rules_ac

> Vec<models::AutocompleteRule> get_rules_ac(x_trace_id, query, limit)
Returns all rules of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteRule>**](AutocompleteRule.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriptions_ac

> Vec<models::AutocompleteBill> get_subscriptions_ac(x_trace_id, query, limit)
Returns all subscriptions of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteBill>**](AutocompleteBill.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag_ac

> Vec<models::AutocompleteTag> get_tag_ac(x_trace_id, query, limit)
Returns all tags of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteTag>**](AutocompleteTag.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_types_ac

> Vec<models::AutocompleteTransactionType> get_transaction_types_ac(x_trace_id, query, limit)
Returns all transaction types returned in a basic auto-complete array. English only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteTransactionType>**](AutocompleteTransactionType.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_ac

> Vec<models::AutocompleteTransaction> get_transactions_ac(x_trace_id, query, limit)
Returns all transaction descriptions of the user returned in a basic auto-complete array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteTransaction>**](AutocompleteTransaction.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_idac

> Vec<models::AutocompleteTransactionId> get_transactions_idac(x_trace_id, query, limit)
Returns all transactions, complemented with their ID, of the user returned in a basic auto-complete array. This endpoint is DEPRECATED and I suggest you DO NOT use it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**query** | Option<**String**> | The autocomplete search query. |  |
**limit** | Option<**i32**> | The number of items returned. |  |

### Return type

[**Vec<models::AutocompleteTransactionId>**](AutocompleteTransactionID.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

