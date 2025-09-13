use std::path::{Path, PathBuf};
use jsonschema::Validator;
use schemars::{schema_for, JsonSchema};
use schemars::_private::NoSerialize;
use serde::{Deserialize, Serialize};
use crate::overlay::OverlayPro;
use crate::types::PixelCoords;

// Main schema
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[schemars(rename_all = "snake_case", deny_unknown_fields)]
pub struct Artwork {
	name: String,
	credits: ArtworkCredits,
	coords: ArtworkCoords,
	tile: Option<String>,
	png: String,
	license: Option<String>
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct ArtworkCredits {
	maintainers: Vec<String>,
	artists: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct ArtworkCoords {
	link: String,
	top_left: PixelCoords
}

macro_rules! read_json_with_schema {
    ($kind:ident, $path:expr, $validator:expr) => {
		match read_json_with_schema::<$kind>($path, $validator) {
		    Ok(value) => value,
			Err(err) => {
				eprintln!("{err}");
				continue;
			}
		}
    };
}

/// Generating JSON schema and loading in the artworks
pub fn load_artworks() -> Vec<Artwork> {
	// Generating metadata schema
	let metadata_schema = schema_for!(Artwork);
	let _ = std::fs::write(
		"./artworks/_schemas/metadata.json",
		serde_json::to_string_pretty(&metadata_schema).unwrap()
	);

	// Generating overlay pro schema
	let overlay_schema = schema_for!(OverlayPro);
	let _ = std::fs::write(
		"./artworks/_schemas/overlay.json",
		serde_json::to_string_pretty(&overlay_schema).unwrap()
	);

	// Creating schema validators
	let metadata_validator = jsonschema::validator_for(&metadata_schema.as_value()).unwrap();
	let overlay_validator = jsonschema::validator_for(&overlay_schema.as_value()).unwrap();

	// Loading in the artworks
	let artworks: Vec<Artwork> = Vec::new();
	for dir in std::fs::read_dir("./artworks").expect("Artworks dir didn't exist") {
		let Ok(dir) = dir else { continue };
		let Ok(kind) = dir.file_type() else { continue };
		if !kind.is_dir() { continue };
		if dir.file_name().to_string_lossy().starts_with("_") {
			continue
		}

		// Reading metadata
		let metadata_path = dir.path().join("metadata.json");
		let metadata = read_json_with_schema!(Artwork, &metadata_path, &metadata_validator);
		if !std::fs::exists(&metadata_path).unwrap_or(false) {
			eprintln!("Metadata file at path '{}' does not exist.", &metadata_path.display());
			continue;
		}
		println!("{:#?}\n", &metadata);

		// Reading the overlay
		let overlay_path = dir.path().join("overlay.json");
		if std::fs::exists(&overlay_path).unwrap_or(false) {
			let overlay = read_json_with_schema!(OverlayPro, &overlay_path, &overlay_validator);
			println!("{:#?}\n", &overlay);
		}
	}
	artworks
}

fn read_json_with_schema<T: for<'a> Deserialize<'a> + Serialize>(path: &PathBuf, validator: &Validator) -> Result<T, String> {
	// Reading the file/data
	let format = format!("Failed to read JSON file at path '{}'", &path.display());
	let contents = match std::fs::read_to_string(&path) {
		Ok(value) => value,
		Err(err) => return Err(format!("{format}: {}", err)),
	};
	let data: T = match serde_json::from_str(&contents) {
		Ok(value) => value,
		Err(err) => return Err(format!("{format}: {}", err))
	};

	// Validating the schema
	let mut validation_errors = Vec::new();
	for error in validator.iter_errors(&serde_json::to_value(&data).unwrap()) {
		validation_errors.push(error.to_string());
	}
	if validation_errors.len() > 0 {
		return Err(format!(
			"File '{JsonFile}' does not follow its JSON schema:\n{Err}\n",
			JsonFile = &path.display(),
			Err = validation_errors.join("\n\t")
		));
	}

	Ok(data)
}