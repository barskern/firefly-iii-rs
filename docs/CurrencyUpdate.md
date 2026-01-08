# CurrencyUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | If the currency is enabled | [optional]
**primary** | Option<**bool**> | If the currency must be the primary for the user. You can only submit TRUE. Submitting FALSE will not drop this currency as the primary currency, because then the system would be without one. | [optional]
**code** | Option<**String**> | The currency code | [optional]
**name** | Option<**String**> | The currency name | [optional]
**symbol** | Option<**String**> | The currency symbol | [optional]
**decimal_places** | Option<**i32**> | How many decimals to use when displaying this currency. Between 0 and 16. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


