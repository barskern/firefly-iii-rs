# TransactionLinkStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**link_type_id** | **String** | The link type ID to use. You can also use the link_type_name field. | 
**link_type_name** | Option<**String**> | The link type name to use. You can also use the link_type_id field. | [optional]
**inward_id** | **String** | The inward transaction transaction_journal_id for the link. This becomes the 'is paid by' transaction of the set. | 
**outward_id** | **String** | The outward transaction transaction_journal_id for the link. This becomes the 'pays for' transaction of the set. | 
**notes** | Option<**String**> | Optional. Some notes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


