# TransactionSplitStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | [**crate::models::TransactionTypeProperty**](TransactionTypeProperty.md) |  | 
**date** | **String** | Date of the transaction | 
**amount** | **String** | Amount of the transaction. | 
**description** | **String** | Description of the transaction. | 
**order** | Option<**i32**> | Order of this entry in the list of transactions. | [optional]
**currency_id** | Option<**String**> | Currency ID. Default is the source account's currency, or the user's default currency. The value you submit may be overruled by the source or destination account. | [optional]
**currency_code** | Option<**String**> | Currency code. Default is the source account's currency, or the user's default currency. The value you submit may be overruled by the source or destination account. | [optional]
**foreign_amount** | Option<**String**> | The amount in a foreign currency. | [optional]
**foreign_currency_id** | Option<**String**> | Currency ID of the foreign currency. Default is null. Is required when you submit a foreign amount. | [optional]
**foreign_currency_code** | Option<**String**> | Currency code of the foreign currency. Default is NULL. Can be used instead of the foreign_currency_id, but this or the ID is required when submitting a foreign amount. | [optional]
**budget_id** | Option<**String**> | The budget ID for this transaction. | [optional]
**budget_name** | Option<**String**> | The name of the budget to be used. If the budget name is unknown, the ID will be used or the value will be ignored. | [optional][readonly]
**category_id** | Option<**String**> | The category ID for this transaction. | [optional]
**category_name** | Option<**String**> | The name of the category to be used. If the category is unknown, it will be created. If the ID and the name point to different categories, the ID overrules the name. | [optional]
**source_id** | Option<**String**> | ID of the source account. For a withdrawal or a transfer, this must always be an asset account. For deposits, this must be a revenue account. | [optional]
**source_name** | Option<**String**> | Name of the source account. For a withdrawal or a transfer, this must always be an asset account. For deposits, this must be a revenue account. Can be used instead of the source_id. If the transaction is a deposit, the source_name can be filled in freely: the account will be created based on the name. | [optional]
**destination_id** | Option<**String**> | ID of the destination account. For a deposit or a transfer, this must always be an asset account. For withdrawals this must be an expense account. | [optional]
**destination_name** | Option<**String**> | Name of the destination account. You can submit the name instead of the ID. For everything except transfers, the account will be auto-generated if unknown, so submitting a name is enough. | [optional]
**reconciled** | Option<**bool**> | If the transaction has been reconciled already. When you set this, the amount can no longer be edited by the user. | [optional]
**piggy_bank_id** | Option<**i32**> | Optional. Use either this or the piggy_bank_name | [optional]
**piggy_bank_name** | Option<**String**> | Optional. Use either this or the piggy_bank_id | [optional]
**bill_id** | Option<**String**> | Optional. Use either this or the bill_name | [optional]
**bill_name** | Option<**String**> | Optional. Use either this or the bill_id | [optional]
**tags** | Option<**Vec<String>**> | Array of tags. | [optional]
**notes** | Option<**String**> |  | [optional]
**internal_reference** | Option<**String**> | Reference to internal reference of other systems. | [optional]
**external_id** | Option<**String**> | Reference to external ID in other systems. | [optional]
**external_url** | Option<**String**> | External, custom URL for this transaction. | [optional]
**bunq_payment_id** | Option<**String**> | Internal ID of bunq transaction. Field is no longer used but still works. | [optional]
**sepa_cc** | Option<**String**> | SEPA Clearing Code | [optional]
**sepa_ct_op** | Option<**String**> | SEPA Opposing Account Identifier | [optional]
**sepa_ct_id** | Option<**String**> | SEPA end-to-end Identifier | [optional]
**sepa_db** | Option<**String**> | SEPA mandate identifier | [optional]
**sepa_country** | Option<**String**> | SEPA Country | [optional]
**sepa_ep** | Option<**String**> | SEPA External Purpose indicator | [optional]
**sepa_ci** | Option<**String**> | SEPA Creditor Identifier | [optional]
**sepa_batch_id** | Option<**String**> | SEPA Batch ID | [optional]
**interest_date** | Option<**String**> |  | [optional]
**book_date** | Option<**String**> |  | [optional]
**process_date** | Option<**String**> |  | [optional]
**due_date** | Option<**String**> |  | [optional]
**payment_date** | Option<**String**> |  | [optional]
**invoice_date** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


