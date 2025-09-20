mod routes;
mod data;
mod overlay;
mod types;

use std::{collections::HashMap, path::PathBuf};

use crate::{data::{artworks::load_artworks, tags::load_tags}, routes::{arts::ArtsRoute, not_found::NotFoundRoute, overlay_pro::OverlayProRoute, robots::RobotsRoute, search::SearchRoute, sitemap::SitemapRoute}};
use crate::data::Data;
use crate::routes::index::IndexRoute;
use generator::generator::Generator;
use serde::{Deserialize, Serialize};

const ADD_NEW_ARTWORK_URL: &str = "https://forms.gle/TrDTajj9KQ2V2FJ48";
const EDIT_ARTWORK_URL:    &str = "https://forms.gle/s1BzTM9AnPgTBYVPA";

fn main() {
	Generator::new()
		.route::<IndexRoute>("/", "Home")
		.route::<SearchRoute>("/search", "Search")
		.route::<NotFoundRoute>("/404.html", "Page not found")
		.route::<OverlayProRoute>("/overlay-pro", "Overlay Pro")
		.route::<SitemapRoute>("/sitemap.xml", "Sitemap")
		.route::<RobotsRoute>("/robots.txt", "Robots")
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
	let artworks = load_artworks();
	let all_tags = load_tags(&artworks);
	Data { artworks, all_tags }
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
	#[derive(Serialize, Deserialize)]
	struct SearchData {
		pub posts: Vec<Option<(String, String)>>,
		pub names: HashMap<String, usize>,
		pub tags: HashMap<String, Vec<usize>>
	}

	let mut search_map = SearchData { posts: Vec::new(), names: HashMap::new(), tags: HashMap::new() };
	
	// TODO: Properly remove discontinued drawings
	for (i, artwork) in data.artworks.iter().enumerate() {
		let post = match artwork.metadata.discontinued {
			true => None,
			false => Some((artwork.slug.clone(), artwork.metadata.name.clone())),
		};
		search_map.posts.insert(i, post);
		search_map.names.insert(artwork.metadata.name.to_lowercase(), i);
		for tag in &artwork.metadata.tags {
			let tag = tag.to_string();
			if let Some(key) = search_map.tags.get_mut(&tag) {
				key.push(i);
			} else {
				search_map.tags.insert(tag.clone(), vec![i]);
			}
		}

	}

	// Serializing to JSON
	#[cfg(debug_assertions)] let out = serde_json::to_string_pretty(&search_map).unwrap();
	#[cfg(not(debug_assertions))] let out = serde_json::to_string(&search_map).unwrap();
	std::fs::write(build_dir.join("static/data/search.json"), out).unwrap();
}
