# BudgetProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**active** | Option<**bool**> |  | [optional]
**name** | **String** |  | 
**order** | Option<**i32**> |  | [optional][readonly]
**notes** | Option<**String**> |  | [optional]
**auto_budget_type** | Option<[**models::AutoBudgetType**](AutoBudgetType.md)> |  | [optional]
**auto_budget_period** | Option<[**models::AutoBudgetPeriod**](AutoBudgetPeriod.md)> |  | [optional]
**object_group_id** | Option<**String**> | The group ID of the group this object is part of. NULL if no group. | [optional]
**object_group_order** | Option<**i32**> | The order of the group. At least 1, for the highest sorting. | [optional][readonly]
**object_group_title** | Option<**String**> | The name of the group. NULL if no group. | [optional]
**object_has_currency_setting** | Option<**bool**> | Indicates whether the object has a currency setting. If false, the object uses the administration's primary currency. | [optional][readonly]
**currency_id** | Option<**String**> | The currency ID of the currency associated with this object. | [optional]
**currency_name** | Option<**String**> | The currency name of the currency associated with this object. | [optional]
**currency_code** | Option<**String**> | The currency code of the currency associated with this object. | [optional]
**currency_symbol** | Option<**String**> |  | [optional][readonly]
**currency_decimal_places** | Option<**i32**> |  | [optional][readonly]
**primary_currency_id** | Option<**String**> | The currency ID of the administration's primary currency. | [optional][readonly]
**primary_currency_name** | Option<**String**> | The currency name of the administration's primary currency. | [optional][readonly]
**primary_currency_code** | Option<**String**> | The currency code of the administration's primary currency. | [optional][readonly]
**primary_currency_symbol** | Option<**String**> | The currency symbol of the administration's primary currency. | [optional][readonly]
**primary_currency_decimal_places** | Option<**i32**> | The currency decimal places of the administration's primary currency. | [optional][readonly]
**auto_budget_amount** | Option<**String**> | The amount for the auto-budget, if set. | [optional]
**pc_auto_budget_amount** | Option<**String**> | The amount for the auto-budget, if set in the primary currency of the administration. | [optional]
**spent** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> | Information on how much was spent in this budget. Is only filled in when the start and end date are submitted. | [optional][readonly]
**pc_spent** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> | Information on how much was spent in this budget. Is only filled in when the start and end date are submitted. It is converted to the primary currency of the administration. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


