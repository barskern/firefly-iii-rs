# RecurrenceStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | [**crate::models::RecurrenceTransactionType**](RecurrenceTransactionType.md) |  | 
**title** | **String** |  | 
**description** | Option<**String**> | Not to be confused with the description of the actual transaction(s) being created. | [optional]
**first_date** | [**String**](string.md) | First time the recurring transaction will fire. Must be after today. | 
**repeat_until** | Option<[**String**](string.md)> | Date until the recurring transaction can fire. Use either this field or repetitions. | 
**nr_of_repetitions** | Option<**i32**> | Max number of created transactions. Use either this field or repeat_until. | [optional]
**apply_rules** | Option<**bool**> | Whether or not to fire the rules after the creation of a transaction. | [optional]
**active** | Option<**bool**> | If the recurrence is even active. | [optional]
**notes** | Option<**String**> |  | [optional]
**repetitions** | [**Vec<crate::models::RecurrenceRepetitionStore>**](RecurrenceRepetitionStore.md) |  | 
**transactions** | [**Vec<crate::models::RecurrenceTransactionStore>**](RecurrenceTransactionStore.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


