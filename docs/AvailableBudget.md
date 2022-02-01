# AvailableBudget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**currency_id** | Option<**String**> | Use either currency_id or currency_code. | [optional]
**currency_code** | Option<**String**> | Use either currency_id or currency_code. | [optional]
**currency_symbol** | Option<**String**> |  | [optional][readonly]
**currency_decimal_places** | Option<**i32**> |  | [optional][readonly]
**amount** | **String** |  | 
**start** | **String** | Start date of the available budget. | 
**end** | **String** | End date of the available budget. | 
**spent_in_budgets** | Option<[**Vec<crate::models::BudgetSpent>**](BudgetSpent.md)> |  | [optional][readonly]
**spent_outside_budget** | Option<[**Vec<crate::models::BudgetSpent>**](BudgetSpent.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


