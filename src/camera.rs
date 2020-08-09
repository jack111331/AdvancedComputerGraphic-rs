use crate::utility::{
	Position,
};

extern crate image;
use image::{ImageBuffer, RgbImage};

pub struct Camera {
	pub size: (u32, u32),
	pub	eye_coord: Position,
	pub	left_lower: Position,
	pub	left_upper: Position,
	pub	right_lower: Position,
	pub	right_upper: Position,
	pub pixel: RgbImage
}

impl Camera {
	pub fn new(size: (u32, u32), eye_coord: Position, left_lower: Position, left_upper: Position, right_lower: Position, right_upper: Position) -> Camera{
		let pixel: RgbImage = ImageBuffer::new(size.0, size.1);
		Camera {
			size,
			eye_coord,
			left_lower,
			left_upper,
			right_lower,
			right_upper,
			pixel
		}
	}

	pub fn to_ppm(&self, filename: &str) {
		self.pixel.save(filename).unwrap();
	}
}