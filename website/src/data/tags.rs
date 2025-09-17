use std::{collections::HashSet, fmt::Display};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::data::Artwork;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct TagInternal(String);

#[derive(Serialize, Deserialize, PartialEq, Hash, Clone, Debug)]
#[serde(from = "TagInternal", into = "TagInternal")]
pub struct Tag {
    pub kind: Option<String>,
    pub inner: String
}
impl Tag {
    pub fn from_str(string: &str) -> Self {
        let string = string.trim().to_lowercase().replace(' ', "_");
        let split = string.split_once(':');
        if let Some((left, right)) = split {
            Self { kind: Some(left.to_owned()), inner: right.to_owned() }
        } else {
            Self { kind: None, inner: string.to_owned() }
        }
    }
}
impl Eq for Tag {}
impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(kind) = &self.kind {
            write!(f, "{kind}:{}", self.inner)
        } else {
            write!(f, "{}", self.inner)
        }
    }
}
impl From<TagInternal> for Tag {
    fn from(value: TagInternal) -> Self {
        Self::from_str(&value.0)
    }
}
impl From<Tag> for TagInternal {
    fn from(value: Tag) -> Self {
        TagInternal(value.to_string())
    }
}
impl JsonSchema for Tag {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        TagInternal::schema_name()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        TagInternal::json_schema(generator)
    }
}

pub fn load_tags(artworks: &Vec<Artwork>) -> HashSet<Tag> {
    let mut tags = HashSet::with_capacity(artworks.len() * 10);
    for artwork in artworks {
        for tag in &artwork.metadata.tags {
            tags.insert(tag.clone());
        }
    }
    tags
}