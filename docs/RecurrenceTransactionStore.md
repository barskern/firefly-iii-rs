# RecurrenceTransactionStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** |  | 
**amount** | **String** | Amount of the transaction. | 
**foreign_amount** | Option<**String**> | Foreign amount of the transaction. | [optional]
**currency_id** | Option<**String**> | Submit either a currency_id or a currency_code. | [optional]
**currency_code** | Option<**String**> | Submit either a currency_id or a currency_code. | [optional]
**foreign_currency_id** | Option<**String**> | Submit either a foreign_currency_id or a foreign_currency_code, or neither. | [optional]
**foreign_currency_code** | Option<**String**> | Submit either a foreign_currency_id or a foreign_currency_code, or neither. | [optional]
**budget_id** | Option<**String**> | The budget ID for this transaction. | [optional]
**category_id** | Option<**String**> | Category ID for this transaction. | [optional]
**source_id** | **String** | ID of the source account. | 
**destination_id** | **String** | ID of the destination account. | 
**tags** | Option<**Vec<String>**> | Array of tags. | [optional]
**piggy_bank_id** | Option<**String**> | Optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


