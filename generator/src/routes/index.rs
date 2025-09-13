use crate::routes::Route;
use askama::Template;
use crate::template::Renderable;

#[derive(Clone, Template)]
#[template(path = "index.askama")]
struct IndexTemplate {
	pub users: Vec<String>
}

pub struct IndexRoute;
impl Route for IndexRoute {
	fn construct() -> impl Route {
		IndexRoute
	}

	fn build(&self) -> impl Renderable {
		IndexTemplate { users: vec!["For".to_string(), "real".to_string()] }.render()
	}
}
