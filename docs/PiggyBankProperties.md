# PiggyBankProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**percentage** | Option<**i32**> | The percentage of the target amount that has been saved, if a target amount is set. | [optional][readonly]
**start_date** | Option<**String**> | The date you started with this piggy bank. | [optional]
**target_date** | Option<**String**> | The date you intend to finish saving money. | [optional]
**order** | Option<**i32**> |  | [optional]
**active** | Option<**bool**> |  | [optional][readonly]
**notes** | Option<**String**> |  | [optional]
**object_group_id** | Option<**String**> | The group ID of the group this object is part of. NULL if no group. | [optional]
**object_group_order** | Option<**i32**> | The order of the group. At least 1, for the highest sorting. | [optional][readonly]
**object_group_title** | Option<**String**> | The name of the group. NULL if no group. | [optional]
**accounts** | Option<[**Vec<models::PiggyBankAccountRead>**](PiggyBankAccountRead.md)> |  | [optional]
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
**target_amount** | Option<**String**> |  | 
**pc_target_amount** | Option<**String**> | The target amount in the primary currency of the administration. | [optional]
**current_amount** | Option<**String**> |  | [optional]
**pc_current_amount** | Option<**String**> | The current amount in the primary currency of the administration. | [optional]
**left_to_save** | Option<**String**> |  | [optional]
**pc_left_to_save** | Option<**String**> |  | [optional]
**save_per_month** | Option<**String**> |  | [optional][readonly]
**pc_save_per_month** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


