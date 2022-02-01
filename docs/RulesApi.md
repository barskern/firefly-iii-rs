# \RulesApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_rule**](RulesApi.md#delete_rule) | **DELETE** /api/v1/rules/{id} | Delete an rule.
[**fire_rule**](RulesApi.md#fire_rule) | **POST** /api/v1/rules/{id}/trigger | Fire the rule on your transactions.
[**get_rule**](RulesApi.md#get_rule) | **GET** /api/v1/rules/{id} | Get a single rule.
[**list_rule**](RulesApi.md#list_rule) | **GET** /api/v1/rules | List all rules.
[**store_rule**](RulesApi.md#store_rule) | **POST** /api/v1/rules | Store a new rule
[**test_rule**](RulesApi.md#test_rule) | **GET** /api/v1/rules/{id}/test | Test which transactions would be hit by the rule. No changes will be made.
[**update_rule**](RulesApi.md#update_rule) | **PUT** /api/v1/rules/{id} | Update existing rule.



## delete_rule

> delete_rule(id)
Delete an rule.

Delete an rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the rule. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fire_rule

> fire_rule(id, start, end, accounts)
Fire the rule on your transactions.

Fire the rule group on your transactions. Changes will be made by the rules in the group! Limit the result if you want to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the rule. | [required] |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the transactions the actions will be applied to. If the start date is not present, it will be set to one year ago. If you use this field, both the start date and the end date must be present.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the transactions the actions will be applied to. If the end date is not present, it will be set to today. If you use this field, both the start date and the end date must be present.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | Limit the triggering of the rule to these asset accounts or liabilities. Only asset accounts and liabilities will be accepted. Other types will be silently dropped.  |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule

> crate::models::RuleSingle get_rule(id)
Get a single rule.

Get a single rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the object.X | [required] |

### Return type

[**crate::models::RuleSingle**](RuleSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rule

> crate::models::RuleArray list_rule(page)
List all rules.

List all rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::RuleArray**](RuleArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_rule

> crate::models::RuleSingle store_rule(rule_store)
Store a new rule

Creates a new rule. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_store** | [**RuleStore**](RuleStore.md) | JSON array or key=value pairs with the necessary rule information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::RuleSingle**](RuleSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_rule

> crate::models::TransactionArray test_rule(id, start, end, accounts)
Test which transactions would be hit by the rule. No changes will be made.

Test which transactions would be hit by the rule. No changes will be made. Limit the result if you want to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the rule. | [required] |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the transactions the test will be applied to. Both the start date and the end date must be present.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the transactions the test will be applied to. Both the start date and the end date must be present.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | Limit the testing of the rule to these asset accounts or liabilities. Only asset accounts and liabilities will be accepted. Other types will be silently dropped.  |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rule

> crate::models::RuleSingle update_rule(id, rule_update)
Update existing rule.

Update existing rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the object.X | [required] |
**rule_update** | [**RuleUpdate**](RuleUpdate.md) | JSON array with updated rule information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::RuleSingle**](RuleSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

