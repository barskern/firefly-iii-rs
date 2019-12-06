# Rule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**title** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**rule_group_id** | **i32** | ID of the rule group under which the rule must be stored. Either this field or rule_group_title is mandatory. | 
**rule_group_title** | Option<**String**> | Title of the rule group under which the rule must be stored. Either this field or rule_group_id is mandatory. | [optional]
**order** | Option<**i32**> |  | [optional][readonly]
**trigger** | **String** | Which action is necessary for the rule to fire? Use either store-journal or update-journal. | 
**active** | Option<**bool**> | Whether or not the rule is even active. Default is true. | [optional]
**strict** | Option<**bool**> | If the rule is set to be strict, ALL triggers must hit in order for the rule to fire. Otherwise, just one is enough. Default value is true. | [optional]
**stop_processing** | Option<**bool**> | If this value is true and the rule is triggered, other rules  after this one in the group will be skipped. Default value is false. | [optional]
**triggers** | [**Vec<crate::models::RuleTrigger>**](RuleTrigger.md) |  | 
**actions** | [**Vec<crate::models::RuleAction>**](RuleAction.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


