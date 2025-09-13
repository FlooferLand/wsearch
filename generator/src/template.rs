pub trait Renderable {
	fn render_template(self) -> Result<String, String>;
}

impl Renderable for String {
	fn render_template(self) -> Result<String, String> {
		Ok(self)
	}
}
impl Renderable for Result<String, askama::Error> {
	fn render_template(self) -> Result<String, String> {
		self.map_err(|e| e.to_string())
	}
}

