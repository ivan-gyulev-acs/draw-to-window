use std::{
	io::{Error, ErrorKind},
	ops::{Index, IndexMut},
	mem
};
pub use crate::point::{Point, Axis};

pub struct SimpleImage {
	pub pixels: Vec<u32>,
	pub size: Point<usize>
}

impl SimpleImage {
	pub fn new(pixels: Vec<u32>, size: Point<usize>) -> Result<SimpleImage, Error> {
		if pixels.len() != size.x * size.y {
			return Err(Error::new(ErrorKind::InvalidData, format!(
				"resolution({width}x{height}) and data size({size} bytes) mismatch",
				width = size.x,
				height = size.y,
				size = pixels.len()
			)));
		}
		Ok(SimpleImage{pixels,size})
	}
	pub fn reverse(&mut self, axis: Axis) {
		match axis {
			Axis::X => {
				for y in 0..self.size.y {
					for x in 0..self.size.x / 2 {
						self.pixels.swap(
							y * self.size.x + x,
							y * self.size.x + (self.size.x - 1 - x)
						);
					}
				}
			},
			Axis::Y => {
				for y in 0..self.size.y / 2 {
					for x in 0..self.size.x {
						self.pixels.swap(
							y * self.size.x + x,
							(self.size.y - 1 - y) * self.size.x + x
						);
					}
				}
			}
		}
	}
}

impl Index<Point<usize>> for SimpleImage {
    type Output = u32;
	#[inline(always)]
    fn index(&self, index: Point<usize>) -> &Self::Output {
		&self.pixels[index.y * self.size.x + index.x]
    }
}

impl IndexMut<Point<usize>> for SimpleImage {
	#[inline(always)]
    fn index_mut(&mut self, index: Point<usize>) -> &mut Self::Output {
		&mut self.pixels[index.y * self.size.x + index.x]
    }
}