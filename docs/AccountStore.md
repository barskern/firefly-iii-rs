# AccountStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**r#type** | [**models::ShortAccountTypeProperty**](ShortAccountTypeProperty.md) |  | 
**iban** | Option<**String**> |  | [optional]
**bic** | Option<**String**> |  | [optional]
**account_number** | Option<**String**> |  | [optional]
**opening_balance** | Option<**String**> | Represents the opening balance, the initial amount this account holds. | [optional]
**opening_balance_date** | Option<**String**> | Represents the date of the opening balance. | [optional]
**virtual_balance** | Option<**String**> |  | [optional]
**currency_id** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's financial administration's currency. | [optional]
**currency_code** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's financial administration's currency. | [optional]
**active** | Option<**bool**> | If omitted, defaults to true. | [optional][default to true]
**order** | Option<**i32**> | Order of the account | [optional]
**include_net_worth** | Option<**bool**> | If omitted, defaults to true. | [optional][default to true]
**account_role** | Option<[**models::AccountRoleProperty**](AccountRoleProperty.md)> |  | [optional]
**credit_card_type** | Option<[**models::CreditCardTypeProperty**](CreditCardTypeProperty.md)> |  | [optional]
**monthly_payment_date** | Option<**String**> | Mandatory when the account_role is ccAsset. Moment at which CC payment installments are asked for by the bank. | [optional]
**liability_type** | Option<[**models::LiabilityTypeProperty**](LiabilityTypeProperty.md)> |  | [optional]
**liability_direction** | Option<[**models::LiabilityDirectionProperty**](LiabilityDirectionProperty.md)> |  | [optional]
**interest** | Option<**String**> | Mandatory when type is liability. Interest percentage. | [optional][default to 0]
**interest_period** | Option<[**models::InterestPeriodProperty**](InterestPeriodProperty.md)> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**latitude** | Option<**f64**> | Latitude of the accounts's location, if applicable. Can be used to draw a map. | [optional]
**longitude** | Option<**f64**> | Latitude of the accounts's location, if applicable. Can be used to draw a map. | [optional]
**zoom_level** | Option<**i32**> | Zoom level for the map, if drawn. This to set the box right. Unfortunately this is a proprietary value because each map provider has different zoom levels. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


