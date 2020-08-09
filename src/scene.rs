use crate::{
	camera::Camera,
	hittable::Hittable,
	utility::{
		HitRecord,
		Velocity,
		Position
	},
	ray::Ray
};

pub struct Scene {
	pub camera: Camera,
	pub hittable_list: Vec<Box<dyn Hittable>>
}

impl Scene {
	pub fn display_scene(&mut self) {
		// TODO change camera type to Option<Camera>
		for i in 0..self.camera.size.1-1 {
			for j in 0..self.camera.size.0-1 {
				let ray = Ray {
					origin: Position::new((self.camera.eye_coord.0, self.camera.eye_coord.1, self.camera.eye_coord.2)),
					velocity: Velocity::new(
						(
                            self.camera.left_upper.0 + (j as f32)/((self.camera.size.0-1) as f32)*(self.camera.right_upper.0-self.camera.left_upper.0) - self.camera.eye_coord.0,
                            self.camera.left_upper.1 - (i as f32)/((self.camera.size.1-1) as f32)*(self.camera.left_upper.1-self.camera.left_lower.1) - self.camera.eye_coord.1,
                            self.camera.left_upper.2 - self.camera.eye_coord.2
                        )
					)
				};
				let mut record: HitRecord = HitRecord::new(-1.0, Velocity::new((0.0, 0.0, 0.0)));
				let mut color = image::Rgb([0, 0, 0]);
				for hittable in self.hittable_list.iter() {
					if hittable.is_hit(&ray, &mut record) {
						color = image::Rgb([255, 255, 255]);
						break;
					}
				};
				self.camera.pixel.put_pixel(j, i, color); 
			}
		}
	}
}