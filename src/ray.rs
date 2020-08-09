use crate::utility::*;

#[derive(Debug)]
pub struct Ray {
	pub origin: Position,
	pub velocity: Velocity
}

impl Ray {
	pub fn new(origin: Position, velocity: Velocity) -> Self {
		Ray {
			origin,
			velocity			
		}
	}
}

#[test]
fn test_PointAt() {
	// Given
	let ray = Ray {
		origin: Position::new((1.0, 1.5, 1.2)),
		velocity: Velocity::new((2.0, 1.1, 1.3))
	};
	let t = 1.3;

	// when
	let result = ray.point_at(t);

	// then
	let answer = Position::new((3.6, 2.9299998, 2.8899999));
	assert_eq!(result.0, answer.0);
	assert_eq!(result.1, answer.1);
	assert_eq!(result.2, answer.2);
}

impl Ray {
	pub fn point_at(&self, t: f32) -> Position {
		&self.origin + &(&t * &self.velocity)
	}
}