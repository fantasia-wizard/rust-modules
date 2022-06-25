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
			Vec2(target.top_left.0, target.bottom_right.0)) && 
			Vec2(self.top_left.1, self.bottom_right.1).intersects(
				Vec2(target.top_left.1, target.bottom_right.1));
	}
	pub fn display(&self){
		let points_x: Vec<i32> = (self.top_left.0..self.bottom_right.0).collect::<Vec<i32>>();
		let points_y: Vec<i32> = (self.top_left.1..self.bottom_right.1).collect::<Vec<i32>>();
		for y in -10..10{
			for x in -10..10{
				if points_x.contains(&&x) && points_y.contains(&&y){
					print!("{}", '█')
				}
				else{
					print!(" ")
				}
			}
			print!("\n")
		}
	}
	pub fn points(&self) -> Vec<Vec<i32>>{
		let mut points: Vec<Vec<i32>> = [].to_vec();
		for x in self.top_left.0..self.bottom_right.0{
			for y in self.top_left.1..self.bottom_right.1{
				points.push([x, y].to_vec())
			}
		}
		return points
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

//Not working with multiple rectangles yet.  Working on a different implementation.
//pub fn display_rects(rects: Vec<Rectangle>){
//	let mut points_x: Vec<i32> = [].to_vec();
//	let mut points_y: Vec<i32> = [].to_vec();

//	for i in rects{
//		points_x.extend((i.top_left.0..i.bottom_right.0).collect::<Vec<i32>>().iter());
//		points_y.extend((i.top_left.1..i.bottom_right.1).collect::<Vec<i32>>().iter());
//	}
//	for y in -10..10{
//		for x in -10..10{
//			if points_x.contains(&&x) && points_y.contains(&&y){
//				print!("{0}{0}", '█')
//			}
//			else{
//				print!("  ")
//			}
//		}
//		print!("\n")
//	}
//}

pub fn display_rects(rects: Vec<Rectangle>){
	let mut points: Vec<Vec<i32>> = [].to_vec();
	for rect in rects{
		points.extend(rect.points())
	}
		for y in -10..10{
			for x in -10..10{
				if points.contains(&vec![x, y]) {
					print!("{0}{0}", '█')
				}
				else{
					print!("  ")
				}
			}
		print!("\n")
	}
}