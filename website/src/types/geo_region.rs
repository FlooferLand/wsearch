use std::{fmt::Display, str::FromStr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
struct GeographicalRegionInternal {
    pub country: String,
    pub state: String
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(from = "GeographicalRegionInternal", into = "GeographicalRegionInternal")]
pub struct GeographicalRegion {
    pub country: Country,
    pub state: String
}

impl From<GeographicalRegionInternal> for GeographicalRegion {
    fn from(value: GeographicalRegionInternal) -> Self {
        Self {
            country: Country::from_str(&value.country).unwrap(),
            state: value.state.clone()
        }
    }
}

impl From<GeographicalRegion> for GeographicalRegionInternal {
    fn from(value: GeographicalRegion) -> Self {
        Self {
            country: value.country.to_string(),
            state: value.state
        }
    }
}

impl Display for GeographicalRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.country, self.state)
    }
}

#[derive(Debug, Clone)]
pub struct Country {
    inner: String
}
impl Country {
    pub fn from_str(str: &str) -> Result<Self, String> {
        Ok(Self {
            inner: str.to_lowercase()
        })
    }
    pub fn code(&self) -> String {
        return self.inner.clone()
    }
}
impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.to_uppercase())
    }
}
