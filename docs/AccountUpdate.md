# AccountUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**iban** | Option<**String**> |  | [optional]
**bic** | Option<**String**> |  | [optional]
**account_number** | Option<**String**> |  | [optional]
**opening_balance** | Option<**String**> |  | [optional]
**opening_balance_date** | Option<[**String**](string.md)> |  | [optional]
**virtual_balance** | Option<**String**> |  | [optional]
**currency_id** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's default currency. | [optional]
**currency_code** | Option<**String**> | Use either currency_id or currency_code. Defaults to the user's default currency. | [optional]
**active** | Option<**bool**> | If omitted, defaults to true. | [optional][default to true]
**order** | Option<**i32**> | Order of the account | [optional]
**include_net_worth** | Option<**bool**> | If omitted, defaults to true. | [optional][default to true]
**account_role** | Option<[**crate::models::AccountRoleProperty**](AccountRoleProperty.md)> |  | [optional]
**credit_card_type** | Option<[**crate::models::CreditCardType**](CreditCardType.md)> |  | [optional]
**monthly_payment_date** | Option<[**String**](string.md)> | Mandatory when the account_role is ccAsset. Moment at which CC payment installments are asked for by the bank. | [optional]
**liability_type** | Option<[**crate::models::LiabilityType**](LiabilityType.md)> |  | [optional]
**interest** | Option<**String**> | Mandatory when type is liability. Interest percentage. | [optional]
**interest_period** | Option<[**crate::models::InterestPeriod**](InterestPeriod.md)> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**latitude** | Option<**f64**> | Latitude of the account's location, if applicable. Can be used to draw a map. If omitted, the existing location will be kept. If submitted as NULL, the current location will be removed. | [optional]
**longitude** | Option<**f64**> | Latitude of the account's location, if applicable. Can be used to draw a map. If omitted, the existing location will be kept. If submitted as NULL, the current location will be removed. | [optional]
**zoom_level** | Option<**i32**> | Zoom level for the map, if drawn. This to set the box right. Unfortunately this is a proprietary value because each map provider has different zoom levels. If omitted, the existing location will be kept. If submitted as NULL, the current location will be removed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


