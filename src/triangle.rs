use crate::{
	ray::Ray,
	hittable::Hittable,
	utility::{
		dot,
		cross,
		Position,
		HitRecord
	},
};

use	libm::fabsf;

pub struct Triangle {
	pub point: (Position, Position, Position)
}

impl Hittable for Triangle {
	fn is_hit(&self, ray: &Ray, record: &mut HitRecord) -> bool {
		let plane_vector = (&self.point.1 - &self.point.0, &self.point.2 - &self.point.0);
		let h = cross(&ray.velocity, &plane_vector.1);
		let a = dot(&plane_vector.0, &h);
		if fabsf(a) < 1e-6 {
	        return false;
		}
	    let f = 1.0/a;
	    let s = &ray.origin - &self.point.0;
	    let u = f * dot(&s, &h);
	    if u < 0.0 || u > 1.0 {
	    	return false;	
	    }
	    let q = cross(&s, &plane_vector.0);
	    let v = f * dot(&ray.velocity, &q);
	    if v < 0.0 || v + u > 1.0 {
	        return false;	    	
	    }
	    let t = f * dot(&plane_vector.1, &q);
	    if t > 1e-6 {
	        record.t = t;
	        return true;
	    }
	    false
	}
}