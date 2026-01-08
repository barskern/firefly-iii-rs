# RecurrenceProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**r#type** | Option<[**models::RecurrenceTransactionType**](RecurrenceTransactionType.md)> |  | [optional]
**title** | Option<**String**> |  | [optional]
**description** | Option<**String**> | Not to be confused with the description of the actual transaction(s) being created. | [optional]
**first_date** | Option<[**String**](string.md)> | First time the recurring transaction will fire. Must be after today. | [optional]
**latest_date** | Option<[**String**](string.md)> | Last time the recurring transaction has fired. | [optional][readonly]
**repeat_until** | Option<[**String**](string.md)> | Date until the recurring transaction can fire. Use either this field or repetitions. | [optional]
**apply_rules** | Option<**bool**> | Whether or not to fire the rules after the creation of a transaction. | [optional]
**active** | Option<**bool**> | If the recurrence is even active. | [optional]
**nr_of_repetitions** | Option<**i32**> | Max number of created transactions. Use either this field or repeat_until. | [optional]
**notes** | Option<**String**> |  | [optional]
**repetitions** | Option<[**Vec<models::RecurrenceRepetition>**](RecurrenceRepetition.md)> |  | [optional]
**transactions** | Option<[**Vec<models::RecurrenceTransaction>**](RecurrenceTransaction.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


