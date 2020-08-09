# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**_type** | **String** | Can only be one one these account types. import, initial-balance and reconciliation cannot be set manually. | 
**iban** | Option<**String**> |  | [optional]
**bic** | Option<**String**> |  | [optional]
**account_number** | Option<**String**> |  | [optional]
**opening_balance** | Option<**f64**> |  | [optional]
**opening_balance_date** | Option<[**String**](string.md)> |  | [optional]
**virtual_balance** | Option<**String**> |  | [optional]
**current_balance** | Option<**String**> |  | [optional][readonly]
**current_balance_date** | Option<[**String**](string.md)> |  | [optional][readonly]
**currency_id** | Option<**i32**> | Use either currency_id or currency_code. Defaults to the user's default currency. | [optional]
**currency_code** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's default currency. | [optional]
**currency_symbol** | Option<**String**> |  | [optional][readonly]
**currency_decimal_places** | Option<**i32**> |  | [optional][readonly]
**active** | Option<**bool**> | If omitted, defaults to true. | [optional]
**include_net_worth** | Option<**bool**> | If omitted, defaults to true. | [optional]
**account_role** | Option<**String**> | Is only mandatory when the type is asset. | [optional]
**credit_card_type** | Option<**String**> | Mandatory when the account_role is ccAsset. Can only be monthlyFull. | [optional]
**monthly_payment_date** | Option<[**String**](string.md)> | Mandatory when the account_role is ccAsset. Moment at which CC payment installments are asked for by the bank. | [optional]
**liability_type** | Option<**String**> | Mandatory when type is liability. Specifies the exact type. | [optional]
**liability_amount** | Option<**String**> | Mandatory when type is liability. Amount of money in the liability. Must be positive. | [optional]
**liability_start_date** | Option<[**String**](string.md)> | Mandatory when type is liability. Start date for the liability. | [optional]
**interest** | Option<**String**> | Mandatory when type is liability. Interest percentage. | [optional]
**interest_period** | Option<**String**> | Mandatory when type is liability. Period over which the interest is calculated. | [optional]
**notes** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


