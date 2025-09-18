use askama::{DynTemplate, Template};
use generator::routes::Route;
use crate::Data;

#[derive(Template)]
#[template(path = "overlay-pro.askama")]
struct OverlayProTemplate;

pub struct OverlayProRoute;
impl Route<Data> for OverlayProRoute {
	fn construct() -> Self {
		OverlayProRoute
	}

	fn build<'a>(&self, _data: &'a Data) -> Result<Box<dyn DynTemplate + 'a>, String> {
		let built = OverlayProTemplate {};
		Ok(Box::new(built))
	}
}
