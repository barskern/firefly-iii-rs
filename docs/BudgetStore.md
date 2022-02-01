# BudgetStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**active** | Option<**bool**> |  | [optional]
**order** | Option<**i32**> |  | [optional][readonly]
**auto_budget_type** | Option<[**crate::models::AutoBudgetType**](AutoBudgetType.md)> |  | [optional]
**auto_budget_currency_id** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's default currency. | [optional]
**auto_budget_currency_code** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's default currency. | [optional]
**auto_budget_amount** | Option<**String**> |  | [optional]
**auto_budget_period** | Option<[**crate::models::AutoBudgetPeriod**](AutoBudgetPeriod.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


