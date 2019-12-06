# BudgetLimit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**currency_id** | Option<**i32**> | Use either currency_id or currency_code. Defaults to the user's default currency. | [optional]
**currency_code** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's default currency. | [optional]
**currency_symbol** | Option<**String**> |  | [optional][readonly]
**currency_decimal_places** | Option<**i32**> |  | [optional][readonly]
**budget_id** | **i32** | The budget ID of the associated budget. | 
**start** | [**String**](string.md) | Start date of the budget limit. | 
**end** | [**String**](string.md) | End date of the budget limit. | 
**amount** | **f64** |  | 
**spent** | Option<[**Vec<crate::models::BudgetSpent>**](BudgetSpent.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


