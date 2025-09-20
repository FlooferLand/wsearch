use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::data::{artworks::ArtworkMetadata, tags::Tag};
use crate::overlay::OverlayPro;

pub mod artworks;
pub mod tags;

#[derive(Default)]
pub struct Data {
	pub artworks: Vec<Artwork>,
	pub all_tags: HashSet<Tag>
}

#[derive(Serialize, Deserialize)]
pub struct Artwork {
	pub slug: String,
	pub metadata: ArtworkMetadata,
	pub overlay: OverlayPro,
	pub missing_data: Vec<String>
}
