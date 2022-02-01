# TransactionLinkUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**link_type_id** | Option<**String**> | The link type ID to use. Use this field OR use the link_type_name field. | [optional]
**link_type_name** | Option<**String**> | The link type name to use. Use this field OR use the link_type_id field. | [optional]
**inward_id** | Option<**String**> | The inward transaction transaction_journal_id for the link. This becomes the 'is paid by' transaction of the set. | [optional]
**outward_id** | Option<**String**> | The outward transaction transaction_journal_id for the link. This becomes the 'pays for' transaction of the set. | [optional]
**notes** | Option<**String**> | Optional. Some notes. If you submit an empty string the current notes will be removed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


