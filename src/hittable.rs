use crate::{
	ray::Ray,
	utility::HitRecord
};

pub trait Hittable {
	fn is_hit(&self, ray: &Ray, record: &mut HitRecord) -> bool;
}