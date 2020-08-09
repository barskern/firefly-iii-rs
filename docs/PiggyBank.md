# PiggyBank

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**account_id** | **i32** | The ID of the asset account this piggy bank is connected to. | 
**account_name** | Option<**String**> | The name of the asset account this piggy bank is connected to. | [optional][readonly]
**currency_id** | Option<**i32**> |  | [optional][readonly]
**currency_code** | Option<**String**> |  | [optional][readonly]
**currency_symbol** | Option<**String**> |  | [optional][readonly]
**currency_decimal_places** | Option<**i32**> | Number of decimals supported by the currency | [optional][readonly]
**target_amount** | **String** |  | 
**percentage** | Option<**f32**> |  | [optional][readonly]
**current_amount** | Option<**String**> |  | [optional]
**left_to_save** | Option<**f64**> |  | [optional][readonly]
**save_per_month** | Option<**f64**> |  | [optional][readonly]
**start_date** | Option<[**String**](string.md)> | The date you started with this piggy bank. | [optional]
**target_date** | Option<[**String**](string.md)> | The date you intend to finish saving money. | [optional]
**order** | Option<**i32**> |  | [optional]
**active** | Option<**bool**> |  | [optional][readonly]
**notes** | Option<**String**> |  | [optional]
**object_group_id** | Option<**i32**> | The group ID of the group this object is part of. NULL if no group. | [optional]
**object_group_order** | Option<**i32**> | The order of the group. At least 1, for the highest sorting. | [optional]
**object_group_title** | Option<**String**> | The name of the group. NULL if no group. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


