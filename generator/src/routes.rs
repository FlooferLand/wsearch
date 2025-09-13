use crate::template::Renderable;

pub mod index;

pub trait Route {
	fn construct() -> impl Route;
	fn build(&self) -> impl Renderable;
}
