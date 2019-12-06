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
**target_amount** | **f64** |  | 
**percentage** | Option<**f32**> |  | [optional][readonly]
**current_amount** | Option<**f64**> |  | [optional]
**left_to_save** | Option<**f64**> |  | [optional][readonly]
**save_per_month** | Option<**f64**> |  | [optional][readonly]
**start_date** | Option<[**String**](string.md)> | The date you started with this piggy bank. | [optional]
**target_date** | Option<[**String**](string.md)> | The date you intend to finish saving money. | [optional]
**order** | Option<**i32**> |  | [optional][readonly]
**active** | Option<**bool**> |  | [optional][readonly]
**notes** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


