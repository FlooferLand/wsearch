use serde::{Deserialize, Serialize};

use crate::data::artworks::ArtworkMetadata;
use crate::overlay::OverlayPro;

pub mod artworks;

#[derive(Default)]
pub struct Data {
	pub artworks: Vec<Artwork>
}

#[derive(Serialize, Deserialize)]
pub struct Artwork {
	pub slug: String,
	pub metadata: ArtworkMetadata,
	pub overlay: OverlayPro
}
