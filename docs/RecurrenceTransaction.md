# RecurrenceTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** |  | 
**amount** | **String** | Amount of the transaction. | 
**foreign_amount** | Option<**String**> | Foreign amount of the transaction. | [optional]
**currency_id** | Option<**i32**> | Submit either a currency_id or a currency_code. | [optional]
**currency_code** | Option<**String**> | Submit either a currency_id or a currency_code. | [optional]
**currency_symbol** | Option<**String**> |  | [optional][readonly]
**currency_decimal_places** | Option<**i32**> | Number of decimals in the currency | [optional][readonly]
**foreign_currency_id** | Option<**i32**> | Submit either a foreign_currency_id or a foreign_currency_code, or neither. | [optional]
**foreign_currency_code** | Option<**String**> | Submit either a foreign_currency_id or a foreign_currency_code, or neither. | [optional]
**foreign_currency_symbol** | Option<**String**> |  | [optional][readonly]
**foreign_currency_decimal_places** | Option<**i32**> | Number of decimals in the currency | [optional][readonly]
**budget_id** | Option<**i32**> | The budget ID for this transaction. | [optional]
**budget_name** | Option<**String**> | The name of the budget to be used. If the budget name is unknown, the ID will be used or the value will be ignored. | [optional][readonly]
**category_id** | Option<**i32**> | Category ID for this transaction. | [optional]
**category_name** | Option<**String**> | Category name for this transaction. | [optional]
**source_id** | Option<**i32**> | ID of the source account. Submit either this or source_name. | [optional]
**source_name** | Option<**String**> | Name of the source account. Submit either this or source_id. | [optional]
**source_iban** | Option<**String**> |  | [optional][readonly]
**source_type** | Option<[**crate::models::AccountTypeProperty**](AccountTypeProperty.md)> |  | [optional]
**destination_id** | Option<**i32**> | ID of the destination account. Submit either this or destination_name. | [optional]
**destination_name** | Option<**String**> | Name of the destination account. Submit either this or destination_id. | [optional]
**destination_iban** | Option<**String**> |  | [optional][readonly]
**destination_type** | Option<[**crate::models::AccountTypeProperty**](AccountTypeProperty.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | Array of tags. | [optional]
**piggy_bank_id** | Option<**i32**> | Optional. Use either this or the piggy_bank_name | [optional]
**piggy_bank_name** | Option<**String**> | Optional. Use either this or the piggy_bank_id | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


