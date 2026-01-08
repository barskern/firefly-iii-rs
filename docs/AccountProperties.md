# AccountProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**active** | Option<**bool**> |  | [optional][default to true]
**order** | Option<**i32**> | Order of the account. Is NULL if account is not asset or liability. | [optional]
**name** | **String** |  | 
**r#type** | [**models::ShortAccountTypeProperty**](ShortAccountTypeProperty.md) |  | 
**account_role** | Option<[**models::AccountRoleProperty**](AccountRoleProperty.md)> |  | [optional]
**object_group_id** | Option<**String**> | The group ID of the group this object is part of. NULL if no group. | [optional]
**object_group_order** | Option<**i32**> | The order of the group. At least 1, for the highest sorting. | [optional][readonly]
**object_group_title** | Option<**String**> | The name of the group. NULL if no group. | [optional]
**object_has_currency_setting** | Option<**bool**> | Indicates whether the account has a currency setting. If false, the account uses the administration's primary currency. Asset accounts and liability accounts always have a currency setting, while expense and revenue accounts do not. | [optional][readonly]
**currency_id** | Option<**String**> | The currency ID of the currency associated with this object. | [optional]
**currency_name** | Option<**String**> | The currency name of the currency associated with this object. | [optional]
**currency_code** | Option<**String**> | The currency code of the currency associated with this object. | [optional]
**currency_symbol** | Option<**String**> |  | [optional][readonly]
**currency_decimal_places** | Option<**i32**> |  | [optional][readonly]
**primary_currency_id** | Option<**String**> | The currency ID of the administration's primary currency. | [optional][readonly]
**primary_currency_name** | Option<**String**> | The currency name of the administration's primary currency. | [optional][readonly]
**primary_currency_code** | Option<**String**> | The currency code of the administration's primary currency. | [optional][readonly]
**primary_currency_symbol** | Option<**String**> | The currency symbol of the administration's primary currency. | [optional][readonly]
**primary_currency_decimal_places** | Option<**i32**> | The currency decimal places of the administration's primary currency. | [optional][readonly]
**current_balance** | Option<**String**> | The current balance of the account in the account's currency. If the account has no currency, this is the balance in the administration's primary currency. Either way, the `currency_*` fields reflect the currency used. | [optional][readonly]
**pc_current_balance** | Option<**String**> | The current balance of the account in the administration's primary currency. The `primary_currency_*` fields reflect the currency used. This field is NULL if the user does have 'convert to primary' set to true in their settings. | [optional][readonly]
**balance_difference** | Option<**String**> | If you submit a start AND end date, this will be the difference between those two moments. | [optional][readonly]
**pc_balance_difference** | Option<**String**> | If you submit a start AND end date, this will be the difference in the currency of the account or the administration's primary currency between those two moments. | [optional][readonly]
**opening_balance** | Option<**String**> | Represents the opening balance, the initial amount this account holds in the currency of the account or the administration's primary currency if the account has no currency. Either way, the `currency_*` fields reflect the currency used. | [optional]
**pc_opening_balance** | Option<**String**> | The opening balance of the account in the administration's primary currency (pc). The `primary_currency_*` fields reflect the currency used. This field is NULL if the user does have 'convert to primary' set to true in their settings. | [optional]
**virtual_balance** | Option<**String**> | The virtual balance of the account in the account's currency or the administration's primary currency if the account has no currency. | [optional]
**pc_virtual_balance** | Option<**String**> | The virtual balance of the account in the administration's primary currency (pc). The `primary_currency_*` fields reflect the currency used. This field is NULL if the user does have 'convert to primary' set to true in their settings. | [optional]
**debt_amount** | Option<**String**> | In liability accounts (loans, debts and mortgages), this is the amount of debt in the account's currency (see the `currency_*` fields). In asset accounts, this is NULL. | [optional]
**pc_debt_amount** | Option<**String**> | In liability accounts (loans, debts and mortgages), this is the amount of debt in the administration's primary currency (see the `currency_*` fields. In asset accounts, this is NULL. | [optional]
**current_balance_date** | Option<**String**> | The timestamp for this date is always 23:59:59, to indicate it's the balance at the very END of that particular day. | [optional][readonly]
**notes** | Option<**String**> |  | [optional]
**monthly_payment_date** | Option<**String**> | Mandatory when the account_role is ccAsset. Moment at which CC payment installments are asked for by the bank. | [optional]
**credit_card_type** | Option<[**models::CreditCardTypeProperty**](CreditCardTypeProperty.md)> |  | [optional]
**account_number** | Option<**String**> |  | [optional]
**iban** | Option<**String**> |  | [optional]
**bic** | Option<**String**> |  | [optional]
**opening_balance_date** | Option<**String**> | Represents the date of the opening balance. | [optional]
**liability_type** | Option<[**models::LiabilityTypeProperty**](LiabilityTypeProperty.md)> |  | [optional]
**liability_direction** | Option<[**models::LiabilityDirectionProperty**](LiabilityDirectionProperty.md)> |  | [optional]
**interest** | Option<**String**> | Mandatory when type is liability. Interest percentage. | [optional]
**interest_period** | Option<[**models::InterestPeriodProperty**](InterestPeriodProperty.md)> |  | [optional]
**include_net_worth** | Option<**bool**> |  | [optional][default to true]
**longitude** | Option<**f64**> | Latitude of the accounts's location, if applicable. Can be used to draw a map. | [optional]
**latitude** | Option<**f64**> | Latitude of the accounts's location, if applicable. Can be used to draw a map. | [optional]
**zoom_level** | Option<**i32**> | Zoom level for the map, if drawn. This to set the box right. Unfortunately this is a proprietary value because each map provider has different zoom levels. | [optional]
**last_activity** | Option<**String**> | Last activity of the account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


