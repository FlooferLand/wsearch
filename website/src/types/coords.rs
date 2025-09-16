use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
struct PixelCoordsInternal(i32, i32);

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(from = "PixelCoordsInternal", into = "PixelCoordsInternal")]
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
impl From<PixelCoordsInternal> for PixelCoords {
	fn from(value: PixelCoordsInternal) -> Self {
		Self { x: value.0, y: value.1 }
	}
}
impl From<PixelCoords> for PixelCoordsInternal {
	fn from(value: PixelCoords) -> Self {
		Self(value.x, value.y)
	}
}
