use askama::{DynTemplate, Template};
use generator::routes::Route;
use crate::{data::Artwork, Data};
use chrono::{DateTime, Utc};

#[derive(Template)]
#[template(path = "sitemap.askama")]
struct SitemapTemplate<'a> {
	artworks: &'a Vec<Artwork>,
	last_updated: DateTime<Utc>
}

// TODO: Make the timestamp accurate to the last regenerated date, and add in dates for generation.
pub struct SitemapRoute;
impl Route<Data> for SitemapRoute {
	fn construct() -> Self {
		SitemapRoute
	}

	fn build<'a>(&self, data: &'a Data) -> Result<Box<dyn DynTemplate + 'a>, String> {
		let built = SitemapTemplate {
			artworks: &data.artworks,
			last_updated: DateTime::from_timestamp(1758477329, 0).unwrap()
		};
		Ok(Box::new(built))
	}
}
