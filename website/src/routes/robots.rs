use askama::{DynTemplate, Template};
use generator::routes::Route;
use crate::{data::Artwork, Data};
use chrono::{DateTime, Utc};

#[derive(Template)]
#[template(path = "robots.txt")]
struct RobotsTemplate<'a> {
	artworks: &'a Vec<Artwork>
}

pub struct RobotsRoute;
impl Route<Data> for RobotsRoute {
	fn construct() -> Self {
		RobotsRoute
	}

	fn build<'a>(&self, data: &'a Data) -> Result<Box<dyn DynTemplate + 'a>, String> {
		let built = RobotsTemplate {
			artworks: &data.artworks
		};
		Ok(Box::new(built))
	}
}
