# \PiggyBanksApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_piggy_bank**](PiggyBanksApi.md#delete_piggy_bank) | **DELETE** /api/v1/piggy_banks/{id} | Delete a piggy bank.
[**get_piggy_bank**](PiggyBanksApi.md#get_piggy_bank) | **GET** /api/v1/piggy_banks/{id} | Get a single piggy bank.
[**list_attachment_by_piggy_bank**](PiggyBanksApi.md#list_attachment_by_piggy_bank) | **GET** /api/v1/piggy_banks/{id}/attachments | Lists all attachments.
[**list_event_by_piggy_bank**](PiggyBanksApi.md#list_event_by_piggy_bank) | **GET** /api/v1/piggy_banks/{id}/events | List all events linked to a piggy bank.
[**list_piggy_bank**](PiggyBanksApi.md#list_piggy_bank) | **GET** /api/v1/piggy_banks | List all piggy banks.
[**store_piggy_bank**](PiggyBanksApi.md#store_piggy_bank) | **POST** /api/v1/piggy_banks | Store a new piggy bank
[**update_piggy_bank**](PiggyBanksApi.md#update_piggy_bank) | **PUT** /api/v1/piggy_banks/{id} | Update existing piggy bank.



## delete_piggy_bank

> delete_piggy_bank(id)
Delete a piggy bank.

Delete a piggy bank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the piggy bank. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_piggy_bank

> crate::models::PiggyBankSingle get_piggy_bank(id)
Get a single piggy bank.

Get a single piggy bank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the piggy bank. | [required] |

### Return type

[**crate::models::PiggyBankSingle**](PiggyBankSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment_by_piggy_bank

> crate::models::AttachmentArray list_attachment_by_piggy_bank(id, page)
Lists all attachments.

Lists all attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the piggy bank. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::AttachmentArray**](AttachmentArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_by_piggy_bank

> crate::models::PiggyBankEventArray list_event_by_piggy_bank(id, page)
List all events linked to a piggy bank.

List all events linked to a piggy bank (adding and removing money).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the piggy bank | [required] |
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::PiggyBankEventArray**](PiggyBankEventArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_piggy_bank

> crate::models::PiggyBankArray list_piggy_bank(page)
List all piggy banks.

List all piggy banks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::PiggyBankArray**](PiggyBankArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_piggy_bank

> crate::models::PiggyBankSingle store_piggy_bank(piggy_bank_store)
Store a new piggy bank

Creates a new piggy bank. The data required can be submitted as a JSON body or as a list of parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**piggy_bank_store** | [**PiggyBankStore**](PiggyBankStore.md) | JSON array or key=value pairs with the necessary piggy bank information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::PiggyBankSingle**](PiggyBankSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_piggy_bank

> crate::models::PiggyBankSingle update_piggy_bank(id, piggy_bank_update)
Update existing piggy bank.

Update existing piggy bank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the piggy bank | [required] |
**piggy_bank_update** | [**PiggyBankUpdate**](PiggyBankUpdate.md) | JSON array with updated piggy bank information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::PiggyBankSingle**](PiggyBankSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

