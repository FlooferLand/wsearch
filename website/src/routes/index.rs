use askama::{DynTemplate, Template};
use generator::routes::Route;
use crate::Data;
use crate::data::Artwork;

#[derive(Template)]
#[template(path = "index.askama")]
struct IndexTemplate<'a> {
	pub artworks: &'a Vec<Artwork>
}

pub struct IndexRoute;
impl Route<Data> for IndexRoute {
	fn construct() -> Self {
		IndexRoute
	}

	fn build<'a>(&self, data: &'a Data) -> Result<Box<dyn DynTemplate + 'a>, String> {
		let built = IndexTemplate { artworks: &data.artworks };
		Ok(Box::new(built))
	}
}
