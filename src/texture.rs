pub use crate::point::Point;
pub use crate::image::SimpleImage;

pub struct Rectangle {
	pub center: Point<f64>,
	pub size: Point<f64>
}

pub struct Texture<'a> {
	pub image: &'a SimpleImage,
	pub sprite: Rectangle,
	pub place: Rectangle
}