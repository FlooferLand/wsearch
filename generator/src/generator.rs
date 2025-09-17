use std::any::Any;
use std::collections::HashMap;
use std::{env, io};
use std::path::{Path, PathBuf};

use crate::routes::{BuiltProceduralRoute, BuiltRoute, BuiltStructuredRoute, ProcRouteBuilder, ProceduralRoute, Route, WebTemplate};

#[derive(Default)]
pub struct Generator<Data> {
	routes: Vec<BuiltRoute<Data>>,
	path_mounts: Vec<(String, String)>,
	touchups: Vec<fn(&PathBuf, &Data)>,
	static_dir: PathBuf,
	styles_dir: PathBuf,
	build_dir: PathBuf,
	data_dir: PathBuf,
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

	pub fn styles_dir(mut self, dir: &str) -> Self {
		self.styles_dir = PathBuf::from(dir.to_owned());
		self
	}

	pub fn build_dir(mut self, dir: &str) -> Self {
		self.build_dir = PathBuf::from(dir.to_owned());
		self
	}

	pub fn data_dir(mut self, dir: &str) -> Self {
		self.data_dir = PathBuf::from(dir.to_owned());
		self
	}

	pub fn mount(mut self, keyword: &str, url: &str) -> Self {
		self.path_mounts.push((keyword.to_string(), url.to_string()));
		self
	}

	pub fn touchup(mut self, func: fn(&PathBuf, &Data)) -> Self {
		self.touchups.push(func);
		self
	}

	pub fn insert_data(mut self, data: Data) -> Self {
		self.global_data = Some(data);
		self
	}

	pub fn route<'b, R>(mut self, path: &'b str, title: &'b str) -> Self where R: Route<Data> + 'static {
		let built = BuiltRoute::Structured(BuiltStructuredRoute {
			path: path.to_string(),
			title: title.to_string(),
			inner: Box::new(R::construct())
		});
		self.routes.push(built);
		self
	}

	pub fn proc_route<'b, R>(mut self, path: &'b str, title: &'b str) -> Self where R: ProceduralRoute<Data> + 'static {
		let built = BuiltRoute::Procedural(BuiltProceduralRoute {
			path: path.to_string(),
			title: title.to_string(),
			inner: Box::new(R::construct())
		});
		self.routes.push(built);
		self
	}

	pub fn build(self) {
		// Setup
		let _ = std::fs::create_dir_all(&self.build_dir);

		// Static dir
		let target_static_dir = self.build_dir.join("static/");
		if should_replace(&self.static_dir, &target_static_dir) {
			if std::fs::exists(&target_static_dir).unwrap_or(true) {
				if let Err(err) = std::fs::remove_dir_all(&target_static_dir) {
					eprintln!("Failed to remove static dir '{}': {err}", &target_static_dir.display());
				}
			}
			if let Err(err) = process_and_copy(&self.static_dir, &target_static_dir, &self) {
				eprintln!("Failed to copy over static dir '{}' to '{}': {err}", &self.static_dir.display(), &target_static_dir.display());
			}
		}

		// Data dir
		let target_data_dir = self.build_dir.join("static/data/");
		if should_replace(&self.data_dir, &target_data_dir) {
			if std::fs::exists(&target_data_dir).unwrap_or(true) {
				if let Err(err) = std::fs::remove_dir_all(&target_data_dir) {
					eprintln!("Failed to remove data dir '{}': {err}", &target_data_dir.display());
				}
			}
			if let Err(err) = process_and_copy(&self.data_dir, &target_data_dir, &self) {
				eprintln!("Failed to copy over data dir '{}' to '{}': {err}", &self.data_dir.display(), &target_data_dir.display());
			}
		}

		// Building all the routes
		let data = self.global_data.unwrap();
		for route in &self.routes {
			let route_path = match route {
				BuiltRoute::Structured(route) => route.path.clone(),
				BuiltRoute::Procedural(route) => route.path.clone(),
			};
			match route {
				BuiltRoute::Structured(route) => {
					let template = route.inner.build(&data).expect("Failed building route");
					if !build_route_or_warn(&self.build_dir, &route_path, &template, &self.path_mounts) {
						continue;
					}
				},
				BuiltRoute::Procedural(route) => {
					let mut builder = ProcRouteBuilder::default();
					route.inner.build(&data, &mut builder).expect("Failed building route");
					for (path, _title, template) in &builder.routes {
						if !build_route_or_warn(&self.build_dir, &format!("{route_path}/{path}"), template, &self.path_mounts) {
							continue;
						}
					}
				},
			};
		}

		// Calling all the touchup functions to add more thingies
		for touchup in self.touchups {
			touchup(&self.build_dir, &data);
		}
	}
}

#[allow(unused)]
/// Returns true if a is newer than b
fn should_replace(replacement: &PathBuf, to_be_replaced: &PathBuf) -> bool {
	let Ok(replacement) = std::fs::metadata(replacement)
		.and_then(|m| m.modified())
		else { return true };
	let Ok(to_be_replaced) = std::fs::metadata(to_be_replaced)
		.and_then(|m| m.modified())
		else { return true };
	//replacement > to_be_replaced
	true // TODO: This function is bugged currently, won't always update correctly.
}

/// Recursive function to copy everything from the source folders into the build folder.
/// Also processing any files (ex: scss -> css) whenever found.
fn process_and_copy<Data>(src: &Path, dst: &Path, generator: &Generator<Data>) -> io::Result<()> {
	if src.is_dir() {
		std::fs::create_dir_all(dst)?;
		for entry in std::fs::read_dir(src)? {
			let entry = entry?;
			let src_path = entry.path();
			let dst_path = dst.join(entry.file_name());
			process_and_copy(&src_path, &dst_path, generator)?;
		}
	} else if let Some(ext) = src.extension().and_then(|e| e.to_str()) {
		// Processing and copying over files
		match ext {
			"scss" | "sass" => {
				let code = std::fs::read_to_string(src)?;
				let scss = {
					let options = grass::Options::default()
						.load_path(&generator.styles_dir);
					grass::from_string(code, &options).unwrap()
				};
				std::fs::write(dst.with_extension("css"), scss)?;
			}
			// TODO: Add typescript transformation
			&_ => {
				std::fs::copy(src, dst)?;
			}
		}
	}
	Ok(())
}

// Build and render; Tiny wrapper function to avoid duplicating code.
// Returns true if the render succeeded
fn build_route_or_warn<'a>(
	build_dir: &Path,
	route_path: &str,
	template: &WebTemplate, 
	path_mounts: &Vec<(String, String)>
) -> bool {
	if let Err(err) = render_route(&build_dir, &route_path, &template, &path_mounts) {
		eprintln!("Error building route '{}':\n  - {}", &route_path, err);
		return false;
	}
	return true;
}

/// Renders a route, then writes it to a file
fn render_route<'a>(
	build_dir: &Path,
	route_path: &str,
	template: &WebTemplate, 
	path_mounts: &Vec<(String, String)>
) -> Result<(), String> {
	// Rendering
	let mut values: HashMap<&str, Box<dyn Any>> = HashMap::new();
	values.insert("title", Box::new("wsearch"));
	if cfg!(debug_assertions) {
		values.insert("debug", Box::new(true));
		if env::args().any(|a| a == "--served") {
			values.insert("debug_served", Box::new(true));
		}
	}
	let mut rendered = template.dyn_render_with_values(&values).unwrap();

	// Replacing paths inside render
	for (local, external) in path_mounts {
		let local = if local.chars().nth(0).unwrap() == '$' { &format!("${}", &local[1..]) } else { local };
		let offset = 0;
		while let Some(start) = rendered[offset..].find(local) {
			let Some(end) = rendered[start..].find('"').map(|e| start + e) else { continue };
			
			// Getting the string
			let path = external.to_owned() + &rendered[start+local.len()..end].to_owned();
			let mut path = PathBuf::from(path);

			// Replacing extensions
			if let Some(ext) = path.extension() {
				let ext = ext.to_string_lossy().to_string();
				match ext.as_str() {
					"scss" | "sass" => path = path.with_extension("css"),
					"ts" => path = path.with_extension("js"),
					_ => {}
				}
			}

			rendered.replace_range(start..end, &path.display().to_string());
		}
	}

	// Sanitizing the file path
	let cleaned_route = {
		let mut cleaned_path = PathBuf::from(route_path);
		if cleaned_path.file_name().is_none() && cleaned_path.extension().is_none() {
			cleaned_path = cleaned_path
				.with_file_name("index")
				.with_extension("html");
		} else if cleaned_path.extension().is_none() {
			let _ = std::fs::create_dir_all(&cleaned_path);
			cleaned_path = cleaned_path.join("index.html");
		}
		cleaned_path
	};
	
	// Writing the file
	let out_path = PathBuf::from(format!("{}{}", build_dir.display(), cleaned_route.display()));
	if let Some(parent) = out_path.parent() {
		let _ = std::fs::create_dir_all(parent);
	}
	let write_result = std::fs::write(&out_path, rendered);
	if let Err(err) = write_result {
		return Err(format!("Writing to file '{}' failed: {err}", &out_path.display()));
	}

	Ok(())
}
