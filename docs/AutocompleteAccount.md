# AutocompleteAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** | Name of the account found by an auto-complete search. | 
**name_with_balance** | **String** | Asset accounts and liabilities have a second field with the given date's account balance in the account currency or primary currency. | 
**active** | Option<**bool**> | Is the bill active or not? | [optional]
**r#type** | **String** | Account type of the account found by the auto-complete search. | 
**currency_id** | **String** | ID for the currency used by this account. If the user prefers amounts converted to their primary currency, this primary currency is used instead. | 
**currency_name** | **String** | Currency name for the currency used by this account. If the user prefers amounts converted to their primary currency, this primary currency is used instead. | 
**currency_code** | **String** | Currency code for the currency used by this account. If the user prefers amounts converted to their primary currency, this primary currency is used instead. | 
**currency_symbol** | **String** | Currency symbol for the currency used by this account. If the user prefers amounts converted to their primary currency, this primary currency is used instead. | 
**currency_decimal_places** | **i32** | Number of decimal places for the currency used by this account. If the user prefers amounts converted to their primary currency, this primary currency is used instead. | 
**account_currency_id** | Option<**String**> | ID for the currency used by this account. Even if \"convertToPrimary\" is on, the account currency ID is displayed here. | [optional]
**account_currency_name** | Option<**String**> | Name for the currency used by this account. Even if \"convertToPrimary\" is on, the account currency name is displayed here. | [optional]
**account_currency_code** | Option<**String**> | Code for the currency used by this account. Even if \"convertToPrimary\" is on, the account currency code is displayed here. | [optional]
**account_currency_symbol** | Option<**String**> | Code for the currency used by this account. Even if \"convertToPrimary\" is on, the account currency code is displayed here. | [optional]
**account_currency_decimal_places** | Option<**i32**> | Number of decimal places for the currency used by this account. Even if \"convertToPrimary\" is on, the account currency code is displayed here. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


