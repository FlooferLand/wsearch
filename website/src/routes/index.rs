use askama::{DynTemplate, Template};
use generator::routes::Route;
use crate::Data;

#[derive(Clone, Template)]
#[template(path = "index.askama")]
struct IndexTemplate {
	pub users: Vec<String>
}

pub struct IndexRoute;
impl Route<Data> for IndexRoute {
	fn construct() -> Self {
		IndexRoute
	}

	fn build(&self, data: &Data) -> Result<Box<dyn DynTemplate>, String> {
		let built = IndexTemplate { users: vec!["For".to_string(), "real".to_string()] };
		Ok(Box::new(built))
	}
}
