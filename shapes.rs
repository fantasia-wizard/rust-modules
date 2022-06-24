#![allow(dead_code)]

use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone)]
pub struct Vec2(pub i32, pub i32);

pub struct Rectangle{
	pub name: String,
	pub top_left: Vec2,
	pub bottom_right: Vec2
}

impl Rectangle{
	pub fn area(&self) -> i32{
		return (self.top_left.0 - self.bottom_right.0).abs()*2 +
		(self.top_left.1 - self.bottom_right.1).abs()*2;
	}
	pub fn overlaps(&self, target: Rectangle) -> bool {
		return Vec2(self.top_left.0, self.bottom_right.0).intersects(
			Vec2(target.top_left.0, target.bottom_right.0)) and 
			Vec2(self.top_left.1, self.bottom_right.1).intersects(
				Vec2(target.top_left.1, target.bottom_right.1));
	}
}

impl Vec2{
	pub fn intersects(&self, target: Vec2) -> bool{
		for i in target.0..target.1{
			if (self.0..self.1).contains(&i){
				return true;
			}
		}
		return false;
	}
}

fn square(origin: Vec2, length: i32) -> Rectangle{
	return Rectangle{
		name: "Square".to_string(),
		top_left: origin,
		bottom_right: Vec2(origin.0 + length, origin.1 + length)
	}
}

impl Display for Rectangle{
	fn fmt(&self, f: &mut Formatter) -> Result{
		write!(f, "Name: {}\n\nTop left Vec2: {}\nBottom right Vec2: {}\n\nArea: {}", self.name, self.top_left, self.bottom_right, self.area())
	}
}	

impl Display for Vec2{
	fn fmt(&self, f: &mut Formatter) -> Result{
		write!(f, "({}, {})", self.0, self.1)
	}
}