# RuleTrigger

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**_type** | **String** | The type of thing this trigger responds to. A limited set is possible | 
**value** | **String** | The accompanying value the trigger responds to. This value is often mandatory, but this depends on the trigger. | 
**order** | Option<**i32**> | Order of the trigger | [optional][readonly]
**active** | Option<**bool**> | If the trigger is active. | [optional]
**stop_processing** | Option<**bool**> | When true, other triggers will not be checked if this trigger was triggered. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


