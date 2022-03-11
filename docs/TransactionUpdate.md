# TransactionUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**apply_rules** | Option<**bool**> | Whether or not to apply rules when submitting transaction. | [optional]
**fire_webhooks** | Option<**bool**> | Whether or not to fire the webhooks that are related to this event. | [optional][default to true]
**group_title** | Option<**String**> | Title of the transaction if it has been split in more than one piece. Empty otherwise. | [optional]
**transactions** | Option<[**Vec<crate::models::TransactionSplitUpdate>**](TransactionSplitUpdate.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


