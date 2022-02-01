# BillStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency_id** | Option<**String**> | Use either currency_id or currency_code | [optional]
**currency_code** | Option<**String**> | Use either currency_id or currency_code | [optional]
**name** | **String** |  | 
**amount_min** | **String** |  | 
**amount_max** | **String** |  | 
**date** | **String** |  | 
**end_date** | Option<**String**> | The date after which this bill is no longer valid or applicable | [optional]
**extension_date** | Option<**String**> | The date before which the bill must be renewed (or cancelled) | [optional]
**repeat_freq** | [**crate::models::BillRepeatFrequency**](BillRepeatFrequency.md) |  | 
**skip** | Option<**i32**> | How often the bill must be skipped. 1 means a bi-monthly bill. | [optional]
**active** | Option<**bool**> | If the bill is active. | [optional]
**notes** | Option<**String**> |  | [optional]
**object_group_id** | Option<**String**> | The group ID of the group this object is part of. NULL if no group. | [optional]
**object_group_title** | Option<**String**> | The name of the group. NULL if no group. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


