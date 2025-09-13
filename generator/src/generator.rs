use askama::Template;
use crate::routes::Route;
use crate::template::Renderable;

pub struct RenderContext;

#[derive(Default)]
pub struct Generator {
	routes: Vec<(String, String, String)>
}
impl<'a> Generator {
	pub fn new() -> Self {
		Generator::default()
	}

	pub fn route<R>(mut self, path: &'a str, name: &'a str) -> Self where R: Route {
		let rendered = match R::construct().build().render_template() {
			Ok(value) => value,
			Err(err) => panic!("Failed to render:\n{err}")
		};

		let out = (path.to_string(), name.to_string(), rendered);
		self.routes.push(out);
		self
	}

	pub fn build(self) {
		for route in self.routes {
			let context = RenderContext;
			let rendered = route.2(&context);
			println!("{}", rendered);
		}
	}
}