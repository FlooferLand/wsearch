use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::data::artworks::ArtworkMetadata;

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
	pub fn new(slug: &str, metadata: &ArtworkMetadata) -> Self {
		Self {
			version: 1,
			name: metadata.name.clone(),
			image_url: format!(
				"https://wsearch.flooferland.com/static/data/{Slug}/{PNG}",
				Slug = &slug,
				PNG = &metadata.image.png
			),
			pixel_url: format!(
				"https://backend.wplace.live/s0/pixel/{TileX}/{TileY}?x={X}&y={Y}",
				TileX = metadata.image.tile.x,
				TileY = metadata.image.tile.y,
				X = metadata.image.coords.y,
				Y = metadata.image.coords.y,
			),
			offset_x: 0,
			offset_y: 0,
			opacity: 1.0
		}
	}
}
