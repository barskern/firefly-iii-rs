# WebhookAttempt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**webhook_message_id** | Option<**String**> | The ID of the webhook message this attempt belongs to. | [optional]
**status_code** | Option<**i32**> | The HTTP status code of the error, if any. | [optional]
**logs** | Option<**String**> | Internal log for this attempt. May contain sensitive user data. | [optional]
**response** | Option<**String**> | Webhook receiver response for this attempt, if any. May contain sensitive user data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


