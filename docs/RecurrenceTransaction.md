# RecurrenceTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**description** | **String** |  | 
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
**amount** | **String** | Amount of the transaction. | 
**pc_amount** | Option<**String**> | Amount of the transaction in primary currency. | [optional]
**foreign_amount** | Option<**String**> | Foreign amount of the transaction. | [optional]
**pc_foreign_amount** | Option<**String**> | Foreign amount of the transaction. | [optional]
**foreign_currency_id** | Option<**String**> |  | [optional]
**foreign_currency_name** | Option<**String**> |  | [optional]
**foreign_currency_code** | Option<**String**> |  | [optional]
**foreign_currency_symbol** | Option<**String**> |  | [optional][readonly]
**foreign_currency_decimal_places** | Option<**i32**> | Number of decimals in the currency | [optional][readonly]
**budget_id** | Option<**String**> | The budget ID for this transaction. | [optional]
**budget_name** | Option<**String**> | The name of the budget to be used. If the budget name is unknown, the ID will be used or the value will be ignored. | [optional][readonly]
**category_id** | Option<**String**> | Category ID for this transaction. | [optional]
**category_name** | Option<**String**> | Category name for this transaction. | [optional]
**source_id** | Option<**String**> | ID of the source account. Submit either this or source_name. | [optional]
**source_name** | Option<**String**> | Name of the source account. Submit either this or source_id. | [optional]
**source_iban** | Option<**String**> |  | [optional][readonly]
**source_type** | Option<[**models::AccountTypeProperty**](AccountTypeProperty.md)> |  | [optional]
**destination_id** | Option<**String**> | ID of the destination account. Submit either this or destination_name. | [optional]
**destination_name** | Option<**String**> | Name of the destination account. Submit either this or destination_id. | [optional]
**destination_iban** | Option<**String**> |  | [optional][readonly]
**destination_type** | Option<[**models::AccountTypeProperty**](AccountTypeProperty.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | Array of tags. | [optional]
**piggy_bank_id** | Option<**String**> |  | [optional]
**piggy_bank_name** | Option<**String**> |  | [optional]
**subscription_id** | Option<**String**> |  | [optional]
**subscription_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


