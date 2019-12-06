# TransactionSplit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | Option<**i32**> | User ID | [optional][readonly]
**transaction_journal_id** | Option<**i32**> | ID of the underlying transaction journal. Each transaction consists of a transaction group (see the top ID) and one or more journals making up the splits of the transaction.  | [optional][readonly]
**_type** | Option<**String**> | Type of transaction. expense cannot be written. | [optional]
**date** | [**String**](string.md) | Date of the transaction | 
**amount** | **f64** | Amount of the transaction. | 
**description** | **String** | Description of the transaction. Will only be used if more than one split is submitted. | 
**order** | Option<**i32**> | Order of this entry in the list of transactions. | [optional]
**currency_id** | Option<**i32**> | Currency ID. Default is the source account's currency, or the user's default currency. Can be used instead of currency_code. | [optional]
**currency_code** | Option<**String**> | Currency code. Default is the source account's currency, or the user's default currency. Can be used instead of currency_id. | [optional]
**currency_symbol** | Option<**String**> |  | [optional][readonly]
**currency_name** | Option<**String**> |  | [optional][readonly]
**currency_decimal_places** | Option<**i32**> | Number of decimals used in this currency. | [optional][readonly]
**foreign_amount** | Option<**f64**> | The amount in a foreign currency. | [optional]
**foreign_currency_id** | Option<**i32**> | Currency ID. Default is null. Is required when you submit a foreign amount. | [optional]
**foreign_currency_code** | Option<**String**> | Currency code. Default is NULL. Can be used instead of the foreign_currency_id, but either is required when submitting a foreign amount. | [optional]
**foreign_currency_symbol** | Option<**String**> |  | [optional][readonly]
**foreign_currency_decimal_places** | Option<**i32**> | Number of decimals in the currency | [optional][readonly]
**budget_id** | Option<**i32**> | The budget ID for this transaction. | [optional]
**budget_name** | Option<**String**> | The name of the budget to be used. If the budget name is unknown, the ID will be used or the value will be ignored. | [optional][readonly]
**category_id** | Option<**i32**> | The category ID for this transaction. | [optional]
**category_name** | Option<**String**> | The name of the category to be used. If the category is unknown, it will be created. If the ID and the name point to different categories, the ID overrules the name. | [optional]
**source_id** | **i32** | ID of the source account. For a withdrawal or a transfer, this must always be an asset account. For deposits, this must be a revenue account. | 
**source_name** | Option<**String**> | Name of the source account. For a withdrawal or a transfer, this must always be an asset account. For deposits, this must be a revenue account. Can be used instead of the source_id. If the transaction is a deposit, the source_name can be filled in freely: the account will be created based on the name. | [optional]
**source_iban** | Option<**String**> |  | [optional][readonly]
**source_type** | Option<**String**> |  | [optional][readonly]
**destination_id** | **i32** | ID of the destination account. For a deposit or a transfer, this must always be an asset account. For withdrawals this must be an expense account. | 
**destination_name** | Option<**String**> | Name of the destination account. You can submit the name instead of the ID. For everything except transfers, the account will be auto-generated if unknown, so submitting a name is enough. | [optional]
**destination_iban** | Option<**String**> |  | [optional][readonly]
**destination_type** | Option<**String**> |  | [optional][readonly]
**reconciled** | Option<**bool**> | If the transaction has been reconciled already. When you set this, the amount can no longer be edited by the user. | [optional]
**piggy_bank_id** | Option<**i32**> | Optional. Use either this or the piggy_bank_name | [optional]
**piggy_bank_name** | Option<**String**> | Optional. Use either this or the piggy_bank_id | [optional]
**bill_id** | Option<**i32**> | Optional. Use either this or the bill_name | [optional]
**bill_name** | Option<**String**> | Optional. Use either this or the bill_id | [optional]
**tags** | Option<**Vec<String>**> | Array of tags. | [optional]
**notes** | Option<**String**> |  | [optional]
**internal_reference** | Option<**String**> | Reference to internal reference of other systems. | [optional]
**external_id** | Option<**String**> | Reference to external ID in other systems. | [optional]
**original_source** | Option<**String**> | System generated identifier for original creator of transaction. | [optional][readonly]
**recurrence_id** | Option<**i32**> | Reference to recurrence that made the transaction. | [optional][readonly]
**bunq_payment_id** | Option<**String**> | Internal ID of bunq transaction. | [optional]
**import_hash_v2** | Option<**String**> | Hash value of original import transaction (for duplicate detection). | [optional][readonly]
**sepa_cc** | Option<**String**> | SEPA Clearing Code | [optional]
**sepa_ct_op** | Option<**String**> | SEPA Opposing Account Identifier | [optional]
**sepa_ct_id** | Option<**String**> | SEPA end-to-end Identifier | [optional]
**sepa_db** | Option<**String**> | SEPA mandate identifier | [optional]
**sepa_country** | Option<**String**> | SEPA Country | [optional]
**sepa_ep** | Option<**String**> | SEPA External Purpose indicator | [optional]
**sepa_ci** | Option<**String**> | SEPA Creditor Identifier | [optional]
**sepa_batch_id** | Option<**String**> | SEPA Batch ID | [optional]
**interest_date** | Option<[**String**](string.md)> |  | [optional]
**book_date** | Option<[**String**](string.md)> |  | [optional]
**process_date** | Option<[**String**](string.md)> |  | [optional]
**due_date** | Option<[**String**](string.md)> |  | [optional]
**payment_date** | Option<[**String**](string.md)> |  | [optional]
**invoice_date** | Option<[**String**](string.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


