# \DataApi

All URIs are relative to *https://demo.firefly-iii.org/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_update_transactions**](DataApi.md#bulk_update_transactions) | **POST** /v1/data/bulk/transactions | Bulk update transaction properties. For more information, see https://docs.firefly-iii.org/references/firefly-iii/api/specials/
[**destroy_data**](DataApi.md#destroy_data) | **DELETE** /v1/data/destroy | Endpoint to destroy user data
[**export_accounts**](DataApi.md#export_accounts) | **GET** /v1/data/export/accounts | Export account data from Firefly III
[**export_bills**](DataApi.md#export_bills) | **GET** /v1/data/export/bills | Export bills from Firefly III
[**export_budgets**](DataApi.md#export_budgets) | **GET** /v1/data/export/budgets | Export budgets and budget amount data from Firefly III
[**export_categories**](DataApi.md#export_categories) | **GET** /v1/data/export/categories | Export category data from Firefly III
[**export_piggies**](DataApi.md#export_piggies) | **GET** /v1/data/export/piggy-banks | Export piggy banks from Firefly III
[**export_recurring**](DataApi.md#export_recurring) | **GET** /v1/data/export/recurring | Export recurring transaction data from Firefly III
[**export_rules**](DataApi.md#export_rules) | **GET** /v1/data/export/rules | Export rule groups and rule data from Firefly III
[**export_tags**](DataApi.md#export_tags) | **GET** /v1/data/export/tags | Export tag data from Firefly III
[**export_transactions**](DataApi.md#export_transactions) | **GET** /v1/data/export/transactions | Export transaction data from Firefly III
[**purge_data**](DataApi.md#purge_data) | **DELETE** /v1/data/purge | Endpoint to purge user data



## bulk_update_transactions

> bulk_update_transactions(query, x_trace_id)
Bulk update transaction properties. For more information, see https://docs.firefly-iii.org/references/firefly-iii/api/specials/

Allows you to update transactions in bulk. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | The JSON query. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_data

> destroy_data(objects, x_trace_id)
Endpoint to destroy user data

A call to this endpoint deletes the requested data type. Use it with care and always with user permission. The demo user is incapable of using this endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**objects** | [**DataDestroyObject**](.md) | The type of data that you wish to destroy. You can only use one at a time. | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_accounts

> std::path::PathBuf export_accounts(x_trace_id, r#type)
Export account data from Firefly III

This endpoint allows you to export your accounts from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**r#type** | Option<[**ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_bills

> std::path::PathBuf export_bills(x_trace_id, r#type)
Export bills from Firefly III

This endpoint allows you to export your bills from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**r#type** | Option<[**ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_budgets

> std::path::PathBuf export_budgets(x_trace_id, r#type)
Export budgets and budget amount data from Firefly III

This endpoint allows you to export your budgets and associated budget data from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**r#type** | Option<[**ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_categories

> std::path::PathBuf export_categories(x_trace_id, r#type)
Export category data from Firefly III

This endpoint allows you to export your categories from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**r#type** | Option<[**ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_piggies

> std::path::PathBuf export_piggies(x_trace_id, r#type)
Export piggy banks from Firefly III

This endpoint allows you to export your piggy banks from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**r#type** | Option<[**ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_recurring

> std::path::PathBuf export_recurring(x_trace_id, r#type)
Export recurring transaction data from Firefly III

This endpoint allows you to export your recurring transactions from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**r#type** | Option<[**ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_rules

> std::path::PathBuf export_rules(x_trace_id, r#type)
Export rule groups and rule data from Firefly III

This endpoint allows you to export your rules and rule groups from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**r#type** | Option<[**ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_tags

> std::path::PathBuf export_tags(x_trace_id, r#type)
Export tag data from Firefly III

This endpoint allows you to export your tags from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**r#type** | Option<[**ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_transactions

> std::path::PathBuf export_transactions(start, end, x_trace_id, accounts, r#type)
Export transaction data from Firefly III

This endpoint allows you to export transactions from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |
**accounts** | Option<**String**> | Limit the export of transactions to these accounts only. Only asset accounts will be accepted. Other types will be silently dropped.  |  |
**r#type** | Option<[**ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_data

> purge_data(x_trace_id)
Endpoint to purge user data

A call to this endpoint purges all previously deleted data. Use it with care and always with user permission. The demo user is incapable of using this endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_trace_id** | Option<**uuid::Uuid**> | Unique identifier associated with this request. |  |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth), [local_bearer_auth](../README.md#local_bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

