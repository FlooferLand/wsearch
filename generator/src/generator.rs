use std::any::Any;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::routes::{BuiltRoute, Route};

#[derive(Default)]
pub struct Generator<Data> {
	routes: Vec<BuiltRoute<Data>>,
	static_dir: PathBuf,
	build_dir: PathBuf,
	global_data: Option<Data>,
}
impl<Data: Default> Generator<Data> {
	pub fn new() -> Self {
		Generator::default()
	}

	pub fn static_dir(mut self, dir: &str) -> Self {
		self.static_dir = PathBuf::from(dir.to_owned());
		self
	}

	pub fn build_dir(mut self, dir: &str) -> Self {
		self.build_dir = PathBuf::from(dir.to_owned());
		self
	}

	pub fn data(mut self, data: Data) -> Self {
		self.global_data = Some(data);
		self
	}

	pub fn route<'a, R>(mut self, path: &'a str, title: &'a str) -> Self where R: Route<Data> + 'static {
		let built = BuiltRoute {
			path: path.to_string(),
			title: title.to_string(),
			inner: Box::new(R::construct())
		};
		self.routes.push(built);
		self
	}

	pub fn build(self) {
		// Setup
		let _ = std::fs::create_dir_all(&self.build_dir);

		// Removing existing static directories
		let target_static_dir = self.build_dir.join("static/");
		if let Err(err) = std::fs::remove_dir_all(&target_static_dir) {
			eprintln!("Failed to remove static dir '{}': {err}", &target_static_dir.display());
		}

		// Copying over static dir
		let copy_res = dircpy::CopyBuilder::new(&self.static_dir, &target_static_dir)
			.overwrite_if_newer(true)
			.overwrite_if_size_differs(true)
			.run();
		if let Err(err) = copy_res {
			eprintln!("Failed to copy over static dir '{}' to '{}': {err}", &self.static_dir.display(), &target_static_dir.display());
		}

		// Building all the routes
		let data = self.global_data.unwrap();
		for route in self.routes {
			if let Err(err) = build_route::<Data>(&self.build_dir, &data, &route) {
				eprintln!("Error building route '{}':\n  - {}", &route.path, err);
				continue;
			}
		}
	}
}

fn build_route<Data>(build_dir: &Path, data: &Data, route: &BuiltRoute<Data>) -> Result<(), String> {
	// Building
	let built = match route.inner.build(data) {
		Ok(value) => value,
		Err(err) => return Err(format!("Failed to render:\n{err}"))
	};

	// Rendering
	let mut values: HashMap<&str, Box<dyn Any>> = HashMap::new();
	values.insert("title", Box::new("Real"));
	let rendered = built.dyn_render_with_values(&values).unwrap();

	// Sanitizing the file path
	let cleaned_route = {
		let length = route.path.len();
		let mut cleaned_route = String::with_capacity(length + 10);
		if length == 1 {
			cleaned_route.push_str("index.html");
		}
		cleaned_route
	};

	// Writing the file
	let out_path = build_dir.join(&cleaned_route);
	let write_result = std::fs::write(&out_path, rendered);
	if let Err(err) = write_result {
		return Err(format!("Writing to file '{}' failed: {err}", &out_path.display()));
	}

	Ok(())
}
