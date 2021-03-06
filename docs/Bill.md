# Bill

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**currency_id** | Option<**i32**> | Use either currency_id or currency_code | [optional]
**currency_code** | Option<**String**> | Use either currency_id or currency_code | [optional]
**currency_symbol** | Option<**String**> |  | [optional][readonly]
**currency_decimal_places** | Option<**i32**> |  | [optional][readonly]
**name** | **String** |  | 
**amount_min** | **String** |  | 
**amount_max** | **String** |  | 
**date** | [**String**](string.md) |  | 
**repeat_freq** | **String** | How often the bill must be paid. | 
**skip** | Option<**i32**> | How often the bill must be skipped. 1 means a bi-monthly bill. | [optional]
**active** | Option<**bool**> | If the bill is active. | [optional]
**notes** | Option<**String**> |  | [optional]
**next_expected_match** | Option<[**String**](string.md)> | When the bill is expected to be due. | [optional][readonly]
**object_group_id** | Option<**i32**> | The group ID of the group this object is part of. NULL if no group. | [optional]
**object_group_order** | Option<**i32**> | The order of the group. At least 1, for the highest sorting. | [optional]
**object_group_title** | Option<**String**> | The name of the group. NULL if no group. | [optional]
**pay_dates** | Option<[**Vec<String>**](string.md)> | Array of future dates when the bill is expected to be paid. Autogenerated. | [optional][readonly]
**paid_dates** | Option<[**Vec<crate::models::BillPaidDates>**](Bill_paid_dates.md)> | Array of past transactions when the bill was paid. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


