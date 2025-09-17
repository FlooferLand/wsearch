use std::collections::HashSet;

use askama::{DynTemplate, Template};
use generator::routes::Route;
use crate::{data::tags::Tag, Data};
use crate::data::Artwork;

#[derive(Template)]
#[template(path = "index.askama")]
struct IndexTemplate<'a> {
	pub artworks: &'a Vec<Artwork>,
	pub all_tags: &'a HashSet<Tag>
}

pub struct IndexRoute;
impl Route<Data> for IndexRoute {
	fn construct() -> Self {
		IndexRoute
	}

	fn build<'a>(&self, data: &'a Data) -> Result<Box<dyn DynTemplate + 'a>, String> {
		let built = IndexTemplate { artworks: &data.artworks, all_tags: &data.all_tags };
		Ok(Box::new(built))
	}
}
