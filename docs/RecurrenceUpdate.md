# RecurrenceUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> |  | [optional]
**description** | Option<**String**> | Not to be confused with the description of the actual transaction(s) being created. | [optional]
**first_date** | Option<[**String**](string.md)> | First time the recurring transaction will fire. | [optional]
**repeat_until** | Option<[**String**](string.md)> | Date until the recurring transaction can fire. After that date, it's basically inactive. Use either this field or repetitions. | [optional]
**nr_of_repetitions** | Option<**i32**> | Max number of created transactions. Use either this field or repeat_until. | [optional]
**apply_rules** | Option<**bool**> | Whether or not to fire the rules after the creation of a transaction. | [optional]
**active** | Option<**bool**> | If the recurrence is even active. | [optional]
**notes** | Option<**String**> |  | [optional]
**repetitions** | Option<[**Vec<crate::models::RecurrenceRepetitionUpdate>**](RecurrenceRepetitionUpdate.md)> |  | [optional]
**transactions** | Option<[**Vec<crate::models::RecurrenceTransactionUpdate>**](RecurrenceTransactionUpdate.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


