use askama::Template;
use generator::routes::{ProcRouteBuilder, ProceduralRoute};
use crate::Data;
use crate::data::Artwork;

#[derive(Template)]
#[template(path = "art.askama")]
struct ArtTemplate<'a> {
	pub artwork: &'a Artwork
}

pub struct ArtsRoute;
impl ProceduralRoute<Data> for ArtsRoute {
	fn construct() -> Self {
		ArtsRoute
	}

	fn build<'a>(&self, data: &'a Data, builder: &mut ProcRouteBuilder<'a>) -> Result<(), String> {
        for artwork in &data.artworks {
		    let built = ArtTemplate { artwork: &artwork };
            builder.insert(&artwork.slug, &artwork.metadata.name, Box::new(built));
        }
		Ok(())
	}
}