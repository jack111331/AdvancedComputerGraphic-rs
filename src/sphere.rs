use crate::{
	ray::Ray,
	hittable::Hittable,
	utility::{
		dot,
		HitRecord,
		Position
	},
};

use libm::sqrtf;

pub struct Sphere {
	pub origin: Position,
	pub radius: f32
}

impl Hittable for Sphere {
	fn is_hit(&self, ray: &Ray, record: &mut HitRecord) -> bool {
		let a = dot(&ray.velocity, &ray.velocity);
		let b = 2.0 * dot(&(&ray.origin - &self.origin), &ray.velocity);
		let c = dot(&(&ray.origin - &self.origin), &(&ray.origin - &self.origin)) - self.radius * self.radius; 
		let determinant = b * b - 4.0 * a * c;
	    if determinant > 0.0 {
	        let t = [(-b - sqrtf(determinant)) / 2.0 / a, (-b + sqrtf(determinant)) / 2.0 / a];
	        for &i in t.iter() {
	        	match record.t {
	        		Some(val) if val > i => {
                    	record.t = Some(i);
                    	record.normal = &ray.point_at(i) - &self.origin;
                    	return true;
                	}, None => {
                    	record.t = Some(i);
                    	record.normal = &ray.point_at(i) - &self.origin;
                    	return true;
                	}, Some(_) => {

	        		}
	        	}
	        }
	    }
	    false
    }
}