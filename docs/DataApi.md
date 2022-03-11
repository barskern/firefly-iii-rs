# \DataApi

All URIs are relative to *https://demo.firefly-iii.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_update_transactions**](DataApi.md#bulk_update_transactions) | **POST** /api/v1/data/bulk/transactions | Bulk update transaction properties. For more information, see https://docs.firefly-iii.org/firefly-iii/api/specials
[**destroy_data**](DataApi.md#destroy_data) | **DELETE** /api/v1/data/destroy | Endpoint to destroy user data
[**export_accounts**](DataApi.md#export_accounts) | **GET** /api/v1/data/export/accounts | Export account data from Firefly III
[**export_bills**](DataApi.md#export_bills) | **GET** /api/v1/data/export/bills | Export bills from Firefly III
[**export_budgets**](DataApi.md#export_budgets) | **GET** /api/v1/data/export/budgets | Export budgets and budget amount data from Firefly III
[**export_categories**](DataApi.md#export_categories) | **GET** /api/v1/data/export/categories | Export category data from Firefly III
[**export_piggies**](DataApi.md#export_piggies) | **GET** /api/v1/data/export/piggy-banks | Export piggy banks from Firefly III
[**export_recurring**](DataApi.md#export_recurring) | **GET** /api/v1/data/export/recurring | Export recurring transaction data from Firefly III
[**export_rules**](DataApi.md#export_rules) | **GET** /api/v1/data/export/rules | Export rule groups and rule data from Firefly III
[**export_tags**](DataApi.md#export_tags) | **GET** /api/v1/data/export/tags | Export tag data from Firefly III
[**export_transactions**](DataApi.md#export_transactions) | **GET** /api/v1/data/export/transactions | Export transaction data from Firefly III



## bulk_update_transactions

> bulk_update_transactions(query)
Bulk update transaction properties. For more information, see https://docs.firefly-iii.org/firefly-iii/api/specials

Allows you to update transactions in bulk. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | The JSON query. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_data

> destroy_data(objects)
Endpoint to destroy user data

A call to this endpoint permanently destroys the requested data type. Use it with care and always with user permission. The demo user is incapable of using this endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**objects** | [**crate::models::DataDestroyObject**](.md) | The type of data that you wish to destroy. You can only use one at a time. | [required] |

### Return type

 (empty response body)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_accounts

> std::path::PathBuf export_accounts(_type)
Export account data from Firefly III

This endpoint allows you to export your accounts from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**crate::models::ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_bills

> std::path::PathBuf export_bills(_type)
Export bills from Firefly III

This endpoint allows you to export your bills from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**crate::models::ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_budgets

> std::path::PathBuf export_budgets(_type)
Export budgets and budget amount data from Firefly III

This endpoint allows you to export your budgets and associated budget data from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**crate::models::ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_categories

> std::path::PathBuf export_categories(_type)
Export category data from Firefly III

This endpoint allows you to export your categories from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**crate::models::ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_piggies

> std::path::PathBuf export_piggies(_type)
Export piggy banks from Firefly III

This endpoint allows you to export your piggy banks from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**crate::models::ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_recurring

> std::path::PathBuf export_recurring(_type)
Export recurring transaction data from Firefly III

This endpoint allows you to export your recurring transactions from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**crate::models::ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_rules

> std::path::PathBuf export_rules(_type)
Export rule groups and rule data from Firefly III

This endpoint allows you to export your rules and rule groups from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**crate::models::ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_tags

> std::path::PathBuf export_tags(_type)
Export tag data from Firefly III

This endpoint allows you to export your tags from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**crate::models::ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_transactions

> std::path::PathBuf export_transactions(start, end, accounts, _type)
Export transaction data from Firefly III

This endpoint allows you to export transactions from Firefly III into a file. Currently supports CSV exports only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**end** | **String** | A date formatted YYYY-MM-DD.  | [required] |
**accounts** | Option<**String**> | Limit the export of transactions to these accounts only. Only asset accounts will be accepted. Other types will be silently dropped.  |  |
**_type** | Option<[**crate::models::ExportFileFilter**](.md)> | The file type the export file (CSV is currently the only option). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[firefly_iii_auth](../README.md#firefly_iii_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

