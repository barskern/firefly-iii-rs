# BudgetStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**active** | Option<**bool**> |  | [optional]
**order** | Option<**i32**> |  | [optional][readonly]
**notes** | Option<**String**> |  | [optional]
**fire_webhooks** | Option<**bool**> | Whether or not to fire the webhooks that are related to this event. | [optional][default to true]
**auto_budget_type** | Option<[**models::AutoBudgetType**](AutoBudgetType.md)> |  | [optional]
**auto_budget_currency_id** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's financial administration's currency. | [optional]
**auto_budget_currency_code** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's financial administration's currency. | [optional]
**auto_budget_amount** | Option<**String**> |  | [optional]
**auto_budget_period** | Option<[**models::AutoBudgetPeriod**](AutoBudgetPeriod.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


