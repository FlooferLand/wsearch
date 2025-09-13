use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct PixelCoords {
	pub x: i32,
	pub y: i32
}
impl PixelCoords {
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}
}
