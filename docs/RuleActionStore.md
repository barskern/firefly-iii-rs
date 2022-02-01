# RuleActionStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | [**crate::models::RuleActionKeyword**](RuleActionKeyword.md) |  | 
**value** | Option<**String**> | The accompanying value the action will set, change or update. Can be empty, but for some types this value is mandatory. | 
**order** | Option<**i32**> | Order of the action | [optional]
**active** | Option<**bool**> | If the action is active. Defaults to true. | [optional][default to true]
**stop_processing** | Option<**bool**> | When true, other actions will not be fired after this action has fired. Defaults to false. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


