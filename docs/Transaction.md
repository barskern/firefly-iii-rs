# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**user** | Option<**i32**> | User ID | [optional][readonly]
**error_if_duplicate_hash** | Option<**bool**> | Break if the submitted transaction exists already. | [optional]
**apply_rules** | Option<**bool**> | Whether or not to apply rules when submitting transaction. | [optional]
**group_title** | Option<**String**> | Title of the transaction if it has been split in more than one piece. Empty otherwise. | [optional]
**transactions** | [**Vec<crate::models::TransactionSplit>**](TransactionSplit.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


