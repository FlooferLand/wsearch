use askama::{DynTemplate, Template};
use generator::routes::Route;
use crate::Data;

#[derive(Template)]
#[template(path = "search.askama")]
struct SearchTemplate;

pub struct SearchRoute;
impl Route<Data> for SearchRoute {
	fn construct() -> Self {
		SearchRoute
	}

	fn build<'a>(&self, _data: &'a Data) -> Result<Box<dyn DynTemplate + 'a>, String> {
		let built = SearchTemplate {};
		Ok(Box::new(built))
	}
}
