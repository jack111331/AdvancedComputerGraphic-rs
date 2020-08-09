pub mod scene;
pub mod utility;
pub mod ray;
pub mod hittable;
pub mod camera;
pub mod triangle;
pub mod sphere;
pub mod config;

pub use crate::{
	utility::{Position, Velocity, HitRecord},
	ray::Ray,
	hittable::Hittable,
	camera::Camera,
	sphere::Sphere,
	triangle::Triangle
};