/*
 * Firefly III API v1.5.5
 *
 * This is the documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. You may use the \"Authorize\" button to try the API below. This file was last generated on 2022-01-30T05:47:28+00:00 
 *
 * The version of the OpenAPI document: 1.5.5
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TagModelUpdate {
    /// The tag
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// The date to which the tag is applicable.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Latitude of the tag's location, if applicable. Can be used to draw a map.
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// Latitude of the tag's location, if applicable. Can be used to draw a map.
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Zoom level for the map, if drawn. This to set the box right. Unfortunately this is a proprietary value because each map provider has different zoom levels.
    #[serde(rename = "zoom_level", skip_serializing_if = "Option::is_none")]
    pub zoom_level: Option<i32>,
}

impl TagModelUpdate {
    pub fn new() -> TagModelUpdate {
        TagModelUpdate {
            tag: None,
            date: None,
            description: None,
            latitude: None,
            longitude: None,
            zoom_level: None,
        }
    }
}


