# AvailableBudgetProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
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
**amount** | Option<**String**> | The amount of this available budget in the currency of this available budget. | [optional]
**pc_amount** | Option<**String**> | The amount of this available budget in the primary currency (pc) of this administration. | [optional]
**start** | Option<**String**> | Start date of the available budget. | [optional]
**end** | Option<**String**> | End date of the available budget. | [optional]
**spent_in_budgets** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> |  | [optional][readonly]
**pc_spent_in_budgets** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> | The amount spent in budgets in the primary currency (pc) of this administration.  | [optional][readonly]
**spent_outside_budgets** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> |  | [optional][readonly]
**pc_spent_outside_budgets** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> | The amount spent outside of budgets in the primary currency (pc) of this administration.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


