use crate::data::artworks::Artwork;

pub mod artworks;

#[derive(Default)]
pub struct Data {
	pub artworks: Vec<Artwork>
}
