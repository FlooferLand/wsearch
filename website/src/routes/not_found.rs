use askama::{DynTemplate, Template};
use generator::routes::Route;
use crate::Data;

#[derive(Template)]
#[template(path = "404.askama")]
struct NotFoundTemplate;

pub struct NotFoundRoute;
impl Route<Data> for NotFoundRoute {
	fn construct() -> Self {
		NotFoundRoute
	}

	fn build<'a>(&self, _data: &'a Data) -> Result<Box<dyn DynTemplate + 'a>, String> {
		let built = NotFoundTemplate {};
		Ok(Box::new(built))
	}
}
