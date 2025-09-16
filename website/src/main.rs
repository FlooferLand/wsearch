mod routes;
mod data;
mod overlay;
mod types;

use crate::{data::artworks::load_artworks, routes::arts::ArtsRoute};
use crate::data::Data;
use crate::routes::index::IndexRoute;
use generator::generator::Generator;

fn main() {
	Generator::new()
		.route::<IndexRoute>("/", "Home")
		.proc_route::<ArtsRoute>("/art", "Home")
		.insert_data(load_data())
		.mount("$static", "/static")
		.mount("$scripts", "/static/scripts")
		.mount("$styles", "/static/styles")
		.styles_dir("./website/web/static/styles")
		.static_dir("./website/web/static")
		.data_dir("./artworks")
		.build_dir("./build")
		.build();
}

fn load_data() -> Data {
	Data {
		artworks: load_artworks()
	}
}
