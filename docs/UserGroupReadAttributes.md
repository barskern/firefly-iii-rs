# UserGroupReadAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**in_use** | Option<**bool**> | Is this user group ('financial administration') currently the active administration? | [optional][readonly]
**can_see_members** | Option<**bool**> | Can the current user see the members of this user group? | [optional][readonly]
**title** | Option<**String**> | Title of the user group. By default, it is the same as the user's email address. | [optional]
**primary_currency_id** | Option<**String**> | Returns the primary currency ID of the user group. | [optional][readonly]
**primary_currency_code** | Option<**String**> | Returns the primary currency code of the user group. | [optional]
**primary_currency_symbol** | Option<**String**> | Returns the primary currency symbol of the user group. | [optional][readonly]
**primary_currency_decimal_places** | Option<**i32**> | Returns the primary currency decimal places of the user group. | [optional][readonly]
**members** | Option<[**Vec<models::UserGroupReadMembers>**](UserGroupReadMembers.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


