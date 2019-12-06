# RuleAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**_type** | **String** | The type of thing this action will do. A limited set is possible. | 
**value** | Option<**String**> | The accompanying value the action will set, change or update. Can be empty, but for some types this value is mandatory. | 
**order** | Option<**i32**> | Order of the action | [optional]
**active** | Option<**bool**> | If the action is active. | [optional]
**stop_processing** | Option<**bool**> | When true, other actions will not be fired after this action has fired. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


