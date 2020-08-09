use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
pub struct Position(pub f32, pub f32, pub f32);

#[derive(Debug)]
pub struct Velocity(pub f32, pub f32, pub f32);

pub struct HitRecord {
	// FIXME change t to option f32
	pub t: f32,
	pub normal: Velocity
}

impl Position {
	pub fn new((x, y, z): (f32, f32, f32)) -> Self {
		Position(x, y, z)
	}
}

impl Velocity {
	pub fn new((x, y, z): (f32, f32, f32)) -> Self {
		Velocity(x, y, z)
	}
}

impl HitRecord {
	pub fn new(t: f32, normal: Velocity) -> Self {
		HitRecord {
			t,
			normal
		}
	}
}

impl<'a, 'b> Add<&'b Velocity> for &'a Position {
    type Output = Position;

    fn add(self, rhs: &'b Velocity) -> Position {
    	Position::new((self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2))
    }
}

impl<'a, 'b> Sub<&'b Position> for &'a Position {
    type Output = Velocity;

    fn sub(self, rhs: &'b Position) -> Velocity {
    	Velocity::new((self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2))
    }
}


impl<'a, 'b> Mul<&'b Velocity> for &'a f32 {
    type Output = Velocity;

    fn mul(self, velocity: &'b Velocity) -> Velocity {
    	Velocity::new((self * velocity.0, self * velocity.1, self * velocity.2))
    }
}

pub fn dot(lhs: &Velocity, rhs: &Velocity) -> f32 {
	lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
}

pub fn cross(lhs: &Velocity, rhs: &Velocity) -> Velocity {
	Velocity::new((lhs.1 * rhs.2 - lhs.2 * rhs.1, lhs.2 * rhs.0 - lhs.0 * rhs.2, lhs.0 * rhs.1 - lhs.1 * rhs.0))
}