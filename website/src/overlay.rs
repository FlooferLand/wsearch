use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::types::PixelCoords;

/// Overlay file for Overlay Pro
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct OverlayPro {
	version: i32,
	name: String,
	image_url: String,
	pixel_url: String,
	offset_x: i32,
	offset_y: i32,
	opacity: f32
}
impl OverlayPro {
	fn new(name: String, image_url: String, pixel_url: String, offset: PixelCoords) -> Self {
		Self {
			version: 1,
			name, image_url, pixel_url,
			offset_x: offset.x,
			offset_y: offset.y,
			opacity: 1.0
		}
	}
}
