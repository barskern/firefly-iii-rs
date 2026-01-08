# RuleTriggerStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::RuleTriggerKeyword**](RuleTriggerKeyword.md) |  | 
**value** | **String** | The accompanying value the trigger responds to. This value is often mandatory, but this depends on the trigger. | 
**order** | Option<**i32**> | Order of the trigger | [optional]
**active** | Option<**bool**> | If the trigger is active. Defaults to true. | [optional][default to true]
**prohibited** | Option<**bool**> | If 'prohibited' is true, this rule trigger will be negated. 'Description is' will become 'Description is NOT' etc. | [optional][default to false]
**stop_processing** | Option<**bool**> | When true, other triggers will not be checked if this trigger was triggered. Defaults to false. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


