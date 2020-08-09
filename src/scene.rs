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
	pub camera: Option<Camera>,
	pub hittable_list: Vec<Box<dyn Hittable>>
}

impl Scene {
	pub fn display_scene(&mut self) -> Result<(), &str> {
		match &mut self.camera {
			Some(camera) => {
				for i in 0..camera.size.1-1 {
					for j in 0..camera.size.0-1 {
						let ray = Ray {
							origin: Position::new((camera.eye_coord.0, camera.eye_coord.1, camera.eye_coord.2)),
							velocity: Velocity::new(
								(
		                            camera.left_upper.0 + (j as f32)/((camera.size.0-1) as f32)*(camera.right_upper.0-camera.left_upper.0) - camera.eye_coord.0,
		                            camera.left_upper.1 - (i as f32)/((camera.size.1-1) as f32)*(camera.left_upper.1-camera.left_lower.1) - camera.eye_coord.1,
		                            camera.left_upper.2 - camera.eye_coord.2
		                        )
							)
						};
						let mut record: HitRecord = HitRecord::new(None, Velocity::new((0.0, 0.0, 0.0)));
						let mut color = image::Rgb([0, 0, 0]);
						for hittable in self.hittable_list.iter() {
							if hittable.is_hit(&ray, &mut record) {
								color = image::Rgb([255, 255, 255]);
								break;
							}
						};
						camera.pixel.put_pixel(j, i, color); 
					}
				}
				Ok(())
			}, None => {
				Err("No camera config found")
			}
		}
	}
}