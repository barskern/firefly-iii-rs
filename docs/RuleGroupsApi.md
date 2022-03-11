# \RuleGroupsApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_rule_group**](RuleGroupsApi.md#delete_rule_group) | **DELETE** /api/v1/rule_groups/{id} | Delete a rule group.
[**fire_rule_group**](RuleGroupsApi.md#fire_rule_group) | **POST** /api/v1/rule_groups/{id}/trigger | Fire the rule group on your transactions.
[**get_rule_group**](RuleGroupsApi.md#get_rule_group) | **GET** /api/v1/rule_groups/{id} | Get a single rule group.
[**list_rule_by_group**](RuleGroupsApi.md#list_rule_by_group) | **GET** /api/v1/rule_groups/{id}/rules | List rules in this rule group.
[**list_rule_group**](RuleGroupsApi.md#list_rule_group) | **GET** /api/v1/rule_groups | List all rule groups.
[**store_rule_group**](RuleGroupsApi.md#store_rule_group) | **POST** /api/v1/rule_groups | Store a new rule group.
[**test_rule_group**](RuleGroupsApi.md#test_rule_group) | **GET** /api/v1/rule_groups/{id}/test | Test which transactions would be hit by the rule group. No changes will be made.
[**update_rule_group**](RuleGroupsApi.md#update_rule_group) | **PUT** /api/v1/rule_groups/{id} | Update existing rule group.



## delete_rule_group

> delete_rule_group(id)
Delete a rule group.

Delete a rule group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the rule group. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fire_rule_group

> fire_rule_group(id, start, end, accounts)
Fire the rule group on your transactions.

Fire the rule group on your transactions. Changes will be made by the rules in the rule group! Limit the result if you want to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the rule group. | [required] |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the transactions the actions will be applied to. Both the start date and the end date must be present.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the transactions the actions will be applied to. Both the start date and the end date must be present.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | Limit the triggering of the rule group to these asset accounts or liabilities. Only asset accounts and liabilities will be accepted. Other types will be silently dropped.  |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_group

> crate::models::RuleGroupSingle get_rule_group(id)
Get a single rule group.

Get a single rule group. This does not include the rules. For that, see below.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the rule group. | [required] |

### Return type

[**crate::models::RuleGroupSingle**](RuleGroupSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rule_by_group

> crate::models::RuleArray list_rule_by_group(id, page)
List rules in this rule group.

List rules in this rule group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the rule group. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::RuleArray**](RuleArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rule_group

> crate::models::RuleGroupArray list_rule_group(page)
List all rule groups.

List all rule groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50 |  |

### Return type

[**crate::models::RuleGroupArray**](RuleGroupArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_rule_group

> crate::models::RuleGroupSingle store_rule_group(rule_group_store)
Store a new rule group.

Creates a new rule group. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_group_store** | [**RuleGroupStore**](RuleGroupStore.md) | JSON array or key=value pairs with the necessary rule group information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::RuleGroupSingle**](RuleGroupSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_rule_group

> crate::models::TransactionArray test_rule_group(id, page, start, end, search_limit, triggered_limit, accounts)
Test which transactions would be hit by the rule group. No changes will be made.

Test which transactions would be hit by the rule group. No changes will be made. Limit the result if you want to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the rule group. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50 items. |  |
**start** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the transactions the test will be applied to. Both the start date and the end date must be present.  |  |
**end** | Option<**String**> | A date formatted YYYY-MM-DD, to limit the transactions the test will be applied to. Both the start date and the end date must be present.  |  |
**search_limit** | Option<**i32**> | Maximum number of transactions Firefly III will try. Don't set this too high, or it will take Firefly III very long to run the test. I suggest a max of 200.  |  |
**triggered_limit** | Option<**i32**> | Maximum number of transactions the rule group can actually trigger on, before Firefly III stops. I would suggest setting this to 10 or 15. Don't go above the user's page size, because browsing to page 2 or 3 of a test result would fire the test again, making any navigation efforts very slow.  |  |
**accounts** | Option<[**Vec<i64>**](i64.md)> | Limit the testing of the rule group to these asset accounts or liabilities. Only asset accounts and liabilities will be accepted. Other types will be silently dropped.  |  |

### Return type

[**crate::models::TransactionArray**](TransactionArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rule_group

> crate::models::RuleGroupSingle update_rule_group(id, rule_group_update)
Update existing rule group.

Update existing rule group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the rule group. | [required] |
**rule_group_update** | [**RuleGroupUpdate**](RuleGroupUpdate.md) | JSON array with updated rule group information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::RuleGroupSingle**](RuleGroupSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

