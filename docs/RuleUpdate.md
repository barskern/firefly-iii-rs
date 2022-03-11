# RuleUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**rule_group_id** | Option<**String**> | ID of the rule group under which the rule must be stored. Either this field or rule_group_title is mandatory. | [optional]
**order** | Option<**i32**> |  | [optional]
**trigger** | Option<[**crate::models::RuleTriggerType**](RuleTriggerType.md)> |  | [optional]
**active** | Option<**bool**> | Whether or not the rule is even active. Default is true. | [optional][default to true]
**strict** | Option<**bool**> | If the rule is set to be strict, ALL triggers must hit in order for the rule to fire. Otherwise, just one is enough. Default value is true. | [optional]
**stop_processing** | Option<**bool**> | If this value is true and the rule is triggered, other rules  after this one in the group will be skipped. Default value is false. | [optional][default to false]
**triggers** | Option<[**Vec<crate::models::RuleTriggerUpdate>**](RuleTriggerUpdate.md)> |  | [optional]
**actions** | Option<[**Vec<crate::models::RuleActionUpdate>**](RuleActionUpdate.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


