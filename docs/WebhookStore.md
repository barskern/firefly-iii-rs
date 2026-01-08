# WebhookStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | Option<**bool**> | Boolean to indicate if the webhook is active | [optional]
**title** | **String** | A title for the webhook for easy recognition. | 
**triggers** | Option<[**Vec<models::WebhookTrigger>**](WebhookTrigger.md)> |  | [optional]
**responses** | Option<[**Vec<models::WebhookResponse>**](WebhookResponse.md)> |  | [optional]
**deliveries** | Option<[**Vec<models::WebhookDelivery>**](WebhookDelivery.md)> |  | [optional]
**url** | **String** | The URL of the webhook. Has to start with `https`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


