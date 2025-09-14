mod routes;
mod data;
mod overlay;
mod types;

use crate::data::artworks::load_artworks;
use crate::data::Data;
use crate::routes::index::IndexRoute;
use generator::generator::Generator;

fn main() {
	Generator::new()
		.route::<IndexRoute>("/", "Home")
		.insert_data(load_data())
		.static_dir("./website/web/static")
		.build_dir("./build")
		.build();
}

fn load_data() -> Data {
	Data {
		artworks: load_artworks()
	}
}
