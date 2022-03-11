# \ObjectGroupsApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_group**](ObjectGroupsApi.md#delete_object_group) | **DELETE** /api/v1/object_groups/{id} | Delete a object group.
[**get_object_group**](ObjectGroupsApi.md#get_object_group) | **GET** /api/v1/object_groups/{id} | Get a single object group.
[**list_bill_by_object_group**](ObjectGroupsApi.md#list_bill_by_object_group) | **GET** /api/v1/object_groups/{id}/bills | List all bills with this object group.
[**list_object_groups**](ObjectGroupsApi.md#list_object_groups) | **GET** /api/v1/object_groups | List all oject groups.
[**list_piggy_bank_by_object_group**](ObjectGroupsApi.md#list_piggy_bank_by_object_group) | **GET** /api/v1/object_groups/{id}/piggy_banks | List all piggy banks related to the object group.
[**update_object_group**](ObjectGroupsApi.md#update_object_group) | **PUT** /api/v1/object_groups/{id} | Update existing object group.



## delete_object_group

> delete_object_group(id)
Delete a object group.

Delete a object group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the object group. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_group

> crate::models::ObjectGroupSingle get_object_group(id)
Get a single object group.

Get a single object group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the object group. | [required] |

### Return type

[**crate::models::ObjectGroupSingle**](ObjectGroupSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_bill_by_object_group

> crate::models::BillArray list_bill_by_object_group(id, page)
List all bills with this object group.

List all bills with this object group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**crate::models::BillArray**](BillArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_object_groups

> crate::models::ObjectGroupArray list_object_groups(page)
List all oject groups.

List all oject groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. The default pagination is 50. |  |

### Return type

[**crate::models::ObjectGroupArray**](ObjectGroupArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_piggy_bank_by_object_group

> crate::models::PiggyBankArray list_piggy_bank_by_object_group(id, page)
List all piggy banks related to the object group.

This endpoint returns a list of all the piggy banks connected to the object group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the account. | [required] |
**page** | Option<**i32**> | Page number. The default pagination is per 50 items. |  |

### Return type

[**crate::models::PiggyBankArray**](PiggyBankArray.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_object_group

> crate::models::ObjectGroupSingle update_object_group(id, object_group_update)
Update existing object group.

Update existing object group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the object group | [required] |
**object_group_update** | [**ObjectGroupUpdate**](ObjectGroupUpdate.md) | JSON array with updated piggy bank information. See the model for the exact specifications. | [required] |

### Return type

[**crate::models::ObjectGroupSingle**](ObjectGroupSingle.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/vnd.api+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

