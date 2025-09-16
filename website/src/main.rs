mod routes;
mod data;
mod overlay;
mod types;

use std::{collections::HashMap, path::PathBuf};

use crate::{data::artworks::load_artworks, routes::{arts::ArtsRoute, search::SearchRoute}};
use crate::data::Data;
use crate::routes::index::IndexRoute;
use generator::generator::Generator;

fn main() {
	Generator::new()
		.route::<IndexRoute>("/", "Home")
		.route::<SearchRoute>("/search", "Search")
		.proc_route::<ArtsRoute>("/art", "Home")
		.insert_data(load_data())
		.mount("$static", "/static")
		.mount("$scripts", "/static/scripts")
		.mount("$styles", "/static/styles")
		.mount("$data", "/static/data")
		.styles_dir("./website/web/static/styles")
		.static_dir("./website/web/static")
		.data_dir("./artworks")
		.build_dir("./build")
		.touchup(make_overlay_data)
		.touchup(make_search_data)
		.build();
}

fn load_data() -> Data {
	Data {
		artworks: load_artworks()
	}
}

// TODO: Move this to an "emit side-effect" type system, that automatically writes the fiels to disk into the build folder.
fn make_overlay_data(build_dir: &PathBuf, data: &Data) {
	for artwork in &data.artworks {
		let out = serde_json::to_string_pretty(&artwork.overlay).unwrap();
		std::fs::write(build_dir.join(format!("static/data/{}/overlay.json", &artwork.slug)), out).unwrap();
	}
}

// TODO: Move this to an "emit side-effect" type system, that automatically writes the fiels to disk into the build folder.
fn make_search_data(build_dir: &PathBuf, data: &Data) {
	let mut search_map = HashMap::with_capacity(data.artworks.len());

	for artwork in &data.artworks {
		let key_fields = [
			&artwork.slug,
			&artwork.metadata.name
		];

		let values = [&artwork.slug, &artwork.metadata.name];
		for key in key_fields {
			search_map.insert(key.to_lowercase(), values);
		}
	}

	// Serializing to JSON
	#[cfg(debug_assertions)] let out = serde_json::to_string_pretty(&search_map).unwrap();
	#[cfg(not(debug_assertions))] let out = serde_json::to_string(&search_map).unwrap();
	std::fs::write(build_dir.join("static/data/search.json"), out).unwrap();
}
