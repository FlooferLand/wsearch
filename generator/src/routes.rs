use askama::DynTemplate;

pub type WebTemplate<'a> = Box<dyn DynTemplate + 'a>;

#[derive(Default)]
pub struct ProcRouteBuilder<'a> {
	pub routes: Vec<(String, String, WebTemplate<'a>)>,
}
impl<'a> ProcRouteBuilder<'a> {
	pub fn insert(&mut self, path: &str, title: &str, template: WebTemplate<'a>) {
		self.routes.push((
			path.to_string(),
			title.to_string(),
			template
		));
	}
}

pub enum BuiltRoute<Data> {
	Structured(BuiltStructuredRoute<Data>),
	Procedural(BuiltProceduralRoute<Data>),
}
pub struct BuiltStructuredRoute<Data> {
	pub path: String,
	pub title: String,
	pub inner: Box<dyn Route<Data>>
}
pub struct BuiltProceduralRoute<Data> {
	pub path: String,
	pub title: String,
	pub inner: Box<dyn ProceduralRoute<Data>>
}

pub trait Route<Data> {
	fn construct() -> Self where Self: Sized;
	fn build<'a>(&self, data: &'a Data) -> Result<WebTemplate<'a>, String>;
}

pub trait ProceduralRoute<Data> {
	fn construct() -> Self where Self: Sized;
	fn build<'a>(&self, data: &'a Data, builder: &mut ProcRouteBuilder<'a>) -> Result<(), String>;
}
