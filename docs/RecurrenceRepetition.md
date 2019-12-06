# RecurrenceRepetition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**_type** | **String** | The type of the repetition. ndom means: the n-th weekday of the month, where you can also specify which day of the week. | 
**moment** | **String** | Information that defined the type of repetition. - For 'daily', this is empty. - For 'weekly', it is day of the week between 1 and 7 (Monday - Sunday). - For 'ndom', it is '1,2' or '4,5' or something else, where the first number is the week in the month, and the second number is the day in the week (between 1 and 7). '2,3' means: the 2nd Wednesday of the month - For 'monthly' it is the day of the month (1 - 31) - For yearly, it is a full date, ie '2018-09-17'. The year you use does not matter.  | 
**skip** | Option<**i32**> | How many occurrences to skip. 0 means skip nothing. 1 means every other. | [optional]
**weekend** | Option<**i32**> | How to respond when the recurring transaction falls in the weekend. Possible values: 1. Do nothing, just create it 2. Create no transaction. 3. Skip to the previous Friday. 4. Skip to the next Monday.  | [optional]
**description** | Option<**String**> | Auto-generated repetition description. | [optional][readonly]
**occurrences** | Option<[**Vec<String>**](string.md)> | Array of future dates when the repetition will apply to. Auto generated. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


