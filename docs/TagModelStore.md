# TagModelStore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tag** | **String** | The tag | 
**date** | Option<[**String**](string.md)> | The date to which the tag is applicable. | [optional]
**description** | Option<**String**> |  | [optional]
**latitude** | Option<**f64**> | Latitude of the tag's location, if applicable. Can be used to draw a map. | [optional]
**longitude** | Option<**f64**> | Latitude of the tag's location, if applicable. Can be used to draw a map. | [optional]
**zoom_level** | Option<**i32**> | Zoom level for the map, if drawn. This to set the box right. Unfortunately this is a proprietary value because each map provider has different zoom levels. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


