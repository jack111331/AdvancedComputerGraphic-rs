use std::fs;
use crate::{
	utility::{
		Position
	},
	hittable::Hittable,
	camera::Camera,
	scene::Scene,
	sphere::Sphere,
	triangle::Triangle
};

pub fn create_scene(filename: &str) -> Scene {
	let contents = fs::read_to_string(filename).unwrap();
	let mut size: Option<(u32, u32)> = None;
	let mut eye_coord: Option<Position> = None;
	let mut left_lower: Option<Position> = None;
	let mut left_upper: Option<Position> = None;
	let mut right_lower: Option<Position> = None;
	let mut right_upper: Option<Position> = None;
	let mut hittable_list: Vec<Box<dyn Hittable>> = vec![]; 
	for line in contents.lines() {
		let config: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
		let cmd = &config[0];
		let args = &config[1..];
		match &cmd[..] {
			"E" => {
				let mut it = args.iter().map(|s| s.parse::<f32>().expect("Failed to parse eye coordinate config"));
		        eye_coord = Some(Position::new((it.next().unwrap(), it.next().unwrap(), it.next().unwrap())));
			}, "O" => {
				let mut it = args.iter().map(|s| s.parse::<f32>().expect("Failed to parse canvas config"));
		        left_upper = Some(Position::new((it.next().unwrap(), it.next().unwrap(), it.next().unwrap())));
		        right_upper = Some(Position::new((it.next().unwrap(), it.next().unwrap(), it.next().unwrap())));
		        left_lower = Some(Position::new((it.next().unwrap(), it.next().unwrap(), it.next().unwrap())));
		        right_lower = Some(Position::new((it.next().unwrap(), it.next().unwrap(), it.next().unwrap())));
			}, "R" => {
				let mut it = args.iter().map(|s| s.parse::<u32>().expect("Failed to parse resolution config"));
				size = Some((it.next().unwrap(), it.next().unwrap()));
			}, "S" => {
				let mut it = args.iter().map(|s| s.parse::<f32>().expect("Failed to parse sphere config"));
				let origin = Position::new((it.next().unwrap(), it.next().unwrap(), it.next().unwrap()));
				let radius = it.next().unwrap();
				let sphere = Box::new(Sphere {
					origin,
					radius
				});
				hittable_list.push(sphere);
			}, "T" => {
				let mut it = args.iter().map(|s| s.parse::<f32>().expect("Failed to parse triangle config"));
				let point_1 = Position::new((it.next().unwrap(), it.next().unwrap(), it.next().unwrap()));
				let point_2 = Position::new((it.next().unwrap(), it.next().unwrap(), it.next().unwrap()));
				let point_3 = Position::new((it.next().unwrap(), it.next().unwrap(), it.next().unwrap()));
				let triangle = Box::new(Triangle {
					point: (point_1, point_2, point_3),
				});
				hittable_list.push(triangle);

			}, _ => {
				panic!("Invalid config");
			}
		}
	}

	let camera: Camera = Camera::new(		
		size.expect("Canvas size not in config"),
		eye_coord.expect("Eye coordinate not in config"),
		left_lower.expect("Canvas left lower coordinate not in config"),
		left_upper.expect("Canvas left upper coordinate not in config"),
		right_lower.expect("Canvas right lower coordinate not in config"),
		right_upper.expect("Canvas right upper coordinate not in config")
	);

	let scene: Scene = Scene {
		camera: camera,
		hittable_list
	};

	scene
}
