# Budget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**active** | Option<**bool**> |  | [optional]
**order** | Option<**i32**> |  | [optional][readonly]
**spent** | Option<[**Vec<crate::models::BudgetSpent>**](BudgetSpent.md)> | Information on how much was spent in this budget. Is only filled in when the start and end date are submitted. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


