# \WebhooksApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_webhook**](WebhooksApi.md#delete_webhook) | **DELETE** /api/v1/webhooks/{id} | Delete a webhook.
[**delete_webhook_message**](WebhooksApi.md#delete_webhook_message) | **DELETE** /api/v1/webhooks/{id}/messages/{messageId} | Delete a webhook message.
[**delete_webhook_message_attempt**](WebhooksApi.md#delete_webhook_message_attempt) | **DELETE** /api/v1/webhooks/{id}/messages/{messageId}/attempts/{attemptId} | Delete a webhook attempt.
[**get_single_webhook_message**](WebhooksApi.md#get_single_webhook_message) | **GET** /api/v1/webhooks/{id}/messages/{messageId} | Get a single message from a webhook.
[**get_single_webhook_message_attempt**](WebhooksApi.md#get_single_webhook_message_attempt) | **GET** /api/v1/webhooks/{id}/messages/{messageId}/attempts/{attemptId} | Get a single failed attempt from a single webhook message.
[**get_webhook**](WebhooksApi.md#get_webhook) | **GET** /api/v1/webhooks/{id} | Get a single webhook.
[**get_webhook_message_attempts**](WebhooksApi.md#get_webhook_message_attempts) | **GET** /api/v1/webhooks/{id}/messages/{messageId}/attempts | Get all the failed attempts of a single webhook message.
[**get_webhook_messages**](WebhooksApi.md#get_webhook_messages) | **GET** /api/v1/webhooks/{id}/messages | Get all the messages of a single webhook.
[**list_webhook**](WebhooksApi.md#list_webhook) | **GET** /api/v1/webhooks | List all webhooks.
[**store_webhook**](WebhooksApi.md#store_webhook) | **POST** /api/v1/webhooks | Store a new webhook
[**submit_webook**](WebhooksApi.md#submit_webook) | **POST** /api/v1/webhooks/{id}/submit | Submit messages for a webhook.
[**update_webhook**](WebhooksApi.md#update_webhook) | **PUT** /api/v1/webhooks/{id} | Update existing webhook.



## delete_webhook

> delete_webhook(id)
Delete a webhook.

Delete a webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook ID. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook_message

> delete_webhook_message(id, message_id)
Delete a webhook message.

Delete a webhook message. Any time a webhook is triggered the message is stored before it's sent. You can delete them before or after sending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook ID. | [required] |
**message_id** | **i32** | The webhook message ID. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook_message_attempt

> delete_webhook_message_attempt(id, message_id, attempt_id)
Delete a webhook attempt.

Delete a webhook message attempt. If you delete all attempts for a webhook message, Firefly III will (once again) assume all is well with the webhook message and will try to send it again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook ID. | [required] |
**message_id** | **i32** | The webhook message ID. | [required] |
**attempt_id** | **i32** | The webhook message attempt ID. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_single_webhook_message

> crate::models::WebhookMessageSingle get_single_webhook_message(id, message_id)
Get a single message from a webhook.

When a webhook is triggered it will store the actual content of the webhook in a webhook message. You can view and analyse a single one using this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook ID. | [required] |
**message_id** | **i32** | The webhook message ID. | [required] |

### Return type

[**crate::models::WebhookMessageSingle**](WebhookMessageSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_single_webhook_message_attempt

> crate::models::WebhookAttemptSingle get_single_webhook_message_attempt(id, message_id, attempt_id)
Get a single failed attempt from a single webhook message.

When a webhook message fails to send it will store the failure in an \"attempt\". You can view and analyse these. Webhooks messages that receive too many attempts (failures) will not be fired. You must first clear out old attempts and try again. This endpoint shows you the details of a single attempt. The ID of the attempt must match the corresponding webhook and webhook message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook ID. | [required] |
**message_id** | **i32** | The webhook message ID. | [required] |
**attempt_id** | **i32** | The webhook attempt ID. | [required] |

### Return type

[**crate::models::WebhookAttemptSingle**](WebhookAttemptSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook

> crate::models::WebhookSingle get_webhook(id)
Get a single webhook.

Gets all info of a single webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook ID. | [required] |

### Return type

[**crate::models::WebhookSingle**](WebhookSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_message_attempts

> crate::models::WebhookAttemptArray get_webhook_message_attempts(id, message_id, page)
Get all the failed attempts of a single webhook message.

When a webhook message fails to send it will store the failure in an \"attempt\". You can view and analyse these. Webhook messages that receive too many attempts (failures) will not be sent again. You must first clear out old attempts before the message can go out again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook ID. | [required] |
**message_id** | **i32** | The webhook message ID. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**crate::models::WebhookAttemptArray**](WebhookAttemptArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_messages

> crate::models::WebhookMessageArray get_webhook_messages(id)
Get all the messages of a single webhook.

When a webhook is triggered the actual message that will be send is stored in a \"message\". You can view and analyse these messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook ID. | [required] |

### Return type

[**crate::models::WebhookMessageArray**](WebhookMessageArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhook

> crate::models::WebhookArray list_webhook(page)
List all webhooks.

List all the user's webhooks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number, if necessary. The default pagination is 50, so 50 webhooks per page. |  |

### Return type

[**crate::models::WebhookArray**](WebhookArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_webhook

> crate::models::WebhookSingle store_webhook(webhook_store)
Store a new webhook

Creates a new webhook. The data required can be submitted as a JSON body or as a list of parameters. The webhook will be given a random secret. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_store** | [**WebhookStore**](WebhookStore.md) | JSON array or key=value pairs with the necessary webhook information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::WebhookSingle**](WebhookSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_webook

> submit_webook(id)
Submit messages for a webhook.

This endpoint will submit any open messages for this webhook. This is an asynchronous operation, so you can't see the result. Refresh the webhook message and/or the webhook message attempts to see the results. This may take some time if the webhook receiver is slow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook ID. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook

> crate::models::WebhookSingle update_webhook(id, webhook_update)
Update existing webhook.

Update an existing webhook's information. If you wish to reset the secret, submit any value as the \"secret\". Firefly III will take this as a hint and reset the secret of the webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The webhook ID. | [required] |
**webhook_update** | [**WebhookUpdate**](WebhookUpdate.md) | JSON array with updated webhook information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::WebhookSingle**](WebhookSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

