use std::fmt::Display;

pub enum RenderError {
	Askama(askama::Error),
	String(String),
}
impl Display for RenderError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self {
			RenderError::Askama(ass) => ass.to_string(),
			RenderError::String(string) => string.to_owned()
		};
		write!(f, "{str}")
	}
}
