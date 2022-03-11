# ChartDataSet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | Option<**String**> | This is the title of the current set. It can refer to an account, a budget or another object (by name). | [optional]
**currency_id** | Option<**String**> | The currency ID of the currency associated to the data in the entries. | [optional]
**currency_code** | Option<**String**> |  | [optional]
**currency_symbol** | Option<**String**> |  | [optional]
**currency_decimal_places** | Option<**i32**> | Number of decimals for the currency associated to the data in the entries. | [optional]
**start_date** | Option<**String**> |  | [optional]
**end_date** | Option<**String**> |  | [optional]
**_type** | Option<**String**> | Indicated the type of chart that is expected to be rendered. You can safely ignore this if you want. | [optional]
**y_axis_id** | Option<**i32**> | Used to indicate the Y axis for this data set. Is usually between 0 and 1 (left and right side of the chart). | [optional]
**entries** | Option<[**Vec<crate::models::ChartDataPoint>**](ChartDataPoint.md)> | The actual entries for this data set. They 'key' value is the label for the data point. The value is the actual (numerical) value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


