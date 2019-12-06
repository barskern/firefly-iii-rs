# Attachment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**filename** | **String** |  | 
**attachable_type** | Option<**String**> | The object class to which the attachment must be linked. | [optional][readonly]
**model** | **String** | The object class to which the attachment must be linked. | 
**attachable_id** | Option<**i32**> | ID of the model this attachment is linked to. | [optional][readonly]
**model_id** | **i32** | ID of the model this attachment is linked to. | 
**md5** | Option<**String**> | MD5 hash of the file for basic duplicate detection. | [optional]
**download_uri** | Option<**String**> |  | [optional]
**upload_uri** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**mime** | Option<**String**> |  | [optional][readonly]
**size** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


