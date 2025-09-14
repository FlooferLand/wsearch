use std::any::Any;
use std::collections::HashMap;
use std::{env, io};
use std::path::{Path, PathBuf};
use crate::routes::{BuiltRoute, Route};

#[derive(Default)]
pub struct Generator<Data> {
	routes: Vec<BuiltRoute<Data>>,
	path_mounts: Vec<(String, String)>,
	static_dir: PathBuf,
	build_dir: PathBuf,
	global_data: Option<Data>,
}
impl<'a, Data: Default + 'a> Generator<Data> {
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

	pub fn mount(mut self, keyword: &str, url: &str) -> Self {
		self.path_mounts.push((keyword.to_string(), url.to_string()));
		self
	}

	pub fn insert_data(mut self, data: Data) -> Self {
		self.global_data = Some(data);
		self
	}

	pub fn route<'b, R>(mut self, path: &'b str, title: &'b str) -> Self where R: Route<Data> + 'static {
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
		let copy_res = process_and_copy(&self.static_dir, &target_static_dir);
		if let Err(err) = copy_res {
			eprintln!("Failed to copy over static dir '{}' to '{}': {err}", &self.static_dir.display(), &target_static_dir.display());
		}

		// Building all the routes
		let data = self.global_data.unwrap();
		for route in self.routes {
			if let Err(err) = build_route::<Data>(&self.build_dir, &data, &route, &self.path_mounts) {
				eprintln!("Error building route '{}':\n  - {}", &route.path, err);
				continue;
			}
		}
	}
}

/// Recursive function to copy everything from the source folders into the build folder.
/// Also processing any files (ex: scss -> css) whenever found.
fn process_and_copy(src: &Path, dst: &Path) -> io::Result<()> {
	if src.is_dir() {
		std::fs::create_dir_all(dst)?;
		for entry in std::fs::read_dir(src)? {
			let entry = entry?;
			let src_path = entry.path();
			let dst_path = dst.join(entry.file_name());
			process_and_copy(&src_path, &dst_path)?;
		}
	} else if let Some(ext) = src.extension().and_then(|e| e.to_str()) {
		// Processing and copying over files
		match ext {
			"scss" | "sass" => {
				let code = std::fs::read_to_string(src)?;
				let scss = {
					let options = grass::Options::default();
					grass::from_string(code, &options).unwrap()
				};
				std::fs::write(dst.with_extension("css"), scss)?;
			}
			&_ => {
				std::fs::copy(src, dst)?;
			}
		}
	}
	Ok(())
}

/// Builds and renders a route, then writes it to a file
fn build_route<'a, Data>(
	build_dir: &Path, 
	data: &'a Data, 
	route: &BuiltRoute<Data>, 
	path_mounts: &Vec<(String, String)>
) -> Result<(), String> {
	// Building
	let built = match route.inner.build(data) {
		Ok(value) => value,
		Err(err) => return Err(format!("Failed to render:\n{err}"))
	};

	// Rendering
	let mut values: HashMap<&str, Box<dyn Any>> = HashMap::new();
	values.insert("title", Box::new("Real"));
	if env::args().any(|a| a == "--served") {
		values.insert("debug_served", Box::new(()));
	}
	let mut rendered = built.dyn_render_with_values(&values).unwrap();

	// Replacing paths inside render
	for (local, external) in path_mounts {
		let local = if local.chars().nth(0).unwrap() == '$' { &format!("${}", &local[1..]) } else { local };
		let Some(start) = rendered.find(local) else { continue };
		let Some(end) = rendered[start..].find('"').map(|e| start + e) else { continue };
		
		// Getting the string
		let path = external.to_owned() + &rendered[start+local.len()..end].to_owned();
		let mut path = PathBuf::from(path);

		// Replacing extensions
		if let Some(ext) = path.extension() {
			let ext = ext.to_string_lossy().to_string();
			match ext.as_str() {
				"scss" | "sass" => path = path.with_extension("css"),
				_ => {}
			}
		}

		rendered.replace_range(start..end, &path.display().to_string());
	}

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
