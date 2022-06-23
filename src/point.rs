use std::{
	ops::{
		Add, AddAssign,
		Sub, SubAssign,
		Index,
		IndexMut
	},
	fmt::{self, Display, Formatter},
	marker::{Sized, Copy},
	convert::From,
	slice::Iter
};

// #![feature(adt_const_params)]

#[repr(usize)]
pub enum Axis { X = 0, Y = 1 }
use Axis::{X, Y};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<Type> { 
	pub x: Type,
	pub y: Type
}

// impl Point<f64, 2> {
// 	pub fn rotate(&mut self, angle: f64) {
// 		*self = Point { values: [
// 			self[X] * angle.cos() - self[Y] * angle.sin(),
// 			self[X] * angle.sin() + self[Y] * angle.cos()
// 		]}
// 	}
// }