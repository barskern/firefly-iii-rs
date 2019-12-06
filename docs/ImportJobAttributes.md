# ImportJobAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**tag_id** | Option<**i32**> | ID of the tag related to the import job, if present. | [optional]
**tag_tag** | Option<**String**> | Tag related to the import job, if present. | [optional]
**key** | Option<**String**> | Import job unique identifier. | [optional]
**file_type** | Option<**String**> | File type, if relevant. | [optional]
**provider** | Option<**String**> | Import provider that did the import. | [optional]
**status** | Option<**String**> | Status of import job. | [optional]
**stage** | Option<**String**> | Current stage. | [optional]
**configuration** | Option<**String**> | JSON string with job-specific configuration. | [optional]
**extended_status** | Option<**String**> | JSON string with job-specific status. | [optional]
**transactions** | Option<**String**> | JSON string with a count of transactions in the job. | [optional]
**errors** | Option<**String**> | JSON string with a list of errors. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


