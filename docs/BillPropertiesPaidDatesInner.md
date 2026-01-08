# BillPropertiesPaidDatesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_group_id** | Option<**String**> | Transaction group ID of the transaction linked to this subscription. | [optional][readonly]
**transaction_journal_id** | Option<**String**> | Transaction journal ID of the transaction linked to this subscription. | [optional][readonly]
**date** | Option<**String**> | Date the bill was paid. | [optional][readonly]
**subscription_id** | Option<**String**> | ID of this subscription. | [optional][readonly]
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
**amount** | Option<**String**> | The amount that was paid for this subscription in the subscription's currency. | [optional]
**pc_amount** | Option<**String**> | The amount that was paid for this subscription in the administration's primary currency. | [optional]
**foreign_amount** | Option<**String**> | The foreign amount that was paid for this subscription in the subscription's currency. | [optional]
**pc_foreign_amount** | Option<**String**> | The foreign amount that was paid for this subscription in the administration's primary currency. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


