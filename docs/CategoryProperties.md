# CategoryProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**notes** | Option<**String**> |  | [optional]
**object_has_currency_setting** | Option<**bool**> | This object never has its own currency setting, so this value is always false. | [optional]
**primary_currency_id** | Option<**String**> | The currency ID of the administration's primary currency. | [optional][readonly]
**primary_currency_name** | Option<**String**> | The currency name of the administration's primary currency. | [optional][readonly]
**primary_currency_code** | Option<**String**> | The currency code of the administration's primary currency. | [optional][readonly]
**primary_currency_symbol** | Option<**String**> | The currency symbol of the administration's primary currency. | [optional][readonly]
**primary_currency_decimal_places** | Option<**i32**> | The currency decimal places of the administration's primary currency. | [optional][readonly]
**spent** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> | Amount(s) spent in the currencies in the database for this category. ONLY present when start and date are set. | [optional][readonly]
**pc_spent** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> | Amount(s) spent in the primary currency in the database for this category. ONLY present when start and date are set.  | [optional][readonly]
**earned** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> | Amount(s) earned in the currencies in the database for this category. ONLY present when start and date are set. | [optional][readonly]
**pc_earned** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> | Amount(s) earned in the primary currency in the database for this category. ONLY present when start and date are set.  | [optional][readonly]
**transferred** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> | Amount(s) transferred in the currencies in the database for this category. ONLY present when start and date are set.  | [optional][readonly]
**pc_transferred** | Option<[**Vec<models::ArrayEntryWithCurrencyAndSum>**](ArrayEntryWithCurrencyAndSum.md)> | Amount(s) transferred in primary currency in the database for this category. ONLY present when start and date are set.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


