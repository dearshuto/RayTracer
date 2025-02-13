use crate::traits::{IInnerProduct, IVector3, IVectorComponent3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3f { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn add(&self, rhs: &Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    pub fn dot(&self, other: &Vector3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        Self::new(self.x / norm, self.y / norm, self.z / norm)
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn is_valid(&self) -> bool {
        !self.x.is_nan() && !self.y.is_nan() && !self.z.is_nan()
    }
}

impl std::ops::Add for Vector3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::add(&self, &rhs)
    }
}

impl std::ops::Sub<Vector3f> for Vector3f {
    type Output = Self;

    fn sub(self, rhs: Vector3f) -> Self::Output {
        Vector3f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<Vector3f> for f32 {
    type Output = Vector3f;

    fn mul(self, rhs: Vector3f) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<f32> for Vector3f {
    type Output = Vector3f;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3f::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<f32> for Vector3f {
    type Output = Vector3f;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3f::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl IVectorComponent3<f32> for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3f::new(x, y, z)
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn z(&self) -> f32 {
        self.z
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn set_z(&mut self, z: f32) {
        self.z = z;
    }
}

impl IVector3<f32> for Vector3f {
    fn zero() -> Self {
        Vector3f::zero()
    }

    fn dot(&self, other: &Self) -> f32 {
        self.dot(other)
    }

    fn normalize(&self) -> Self {
        self.normalize()
    }

    fn length(&self) -> f32 {
        self.norm()
    }
}

impl IInnerProduct<f32> for Vector3f {
    fn dot(&self, other: &Self) -> f32 {
        self.dot(other)
    }
}
