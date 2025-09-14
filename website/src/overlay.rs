use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::types::PixelCoords;

/// Overlay file for Overlay Pro
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct OverlayPro {
	pub version: i32,
	pub name: String,
	pub image_url: String,
	pub pixel_url: String,
	pub offset_x: i32,
	pub offset_y: i32,
	pub opacity: f32
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
