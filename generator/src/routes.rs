use askama::DynTemplate;

pub struct BuiltRoute<Data> {
	pub path: String,
	pub title: String,
	pub inner: Box<dyn Route<Data>>
}

pub trait Route<Data> {
	fn construct() -> Self where Self: Sized;
	fn build<'a>(&self, data: &'a Data) -> Result<Box<dyn DynTemplate + 'a>, String>;
}
