use std::fmt::Display;

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
impl Display for PixelCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}
