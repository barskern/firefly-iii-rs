# BasicSummaryEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | Option<**String**> | This is a reference to the type of info shared, not influenced by translations or user preferences. The EUR value is a reference to the currency code. Possibilities are: balance-in-ABC, spent-in-ABC, earned-in-ABC, bills-paid-in-ABC, bills-unpaid-in-ABC, left-to-spend-in-ABC and net-worth-in-ABC. | [optional]
**title** | Option<**String**> | A translated title for the information shared. | [optional]
**monetary_value** | Option<**f64**> | The amount as a float. | [optional]
**currency_id** | Option<**String**> | The currency ID of the associated currency. | [optional]
**currency_code** | Option<**String**> |  | [optional]
**currency_symbol** | Option<**String**> |  | [optional]
**currency_decimal_places** | Option<**i32**> | Number of decimals for the associated currency. | [optional]
**value_parsed** | Option<**String**> | The amount formatted according to the users locale | [optional]
**local_icon** | Option<**String**> | Reference to a font-awesome icon without the fa- part. | [optional]
**sub_title** | Option<**String**> | A short explanation of the amounts origin. Already formatted according to the locale of the user or translated, if relevant. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


