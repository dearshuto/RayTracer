#[derive(Debug, Copy, Clone)]
pub struct Vector3f
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f
{
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3f{x, y, z}
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn add(&self, rhs: &Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    pub fn dot(&self, other: &Vector3f) -> f32 {
        let self_vector = self.to_nalgebra();
        let other_vector = other.to_nalgebra();
        let result = self_vector.dot(&other_vector);
        result
    }

    pub fn normalize(&self) -> Self {
        let result = self.to_nalgebra().normalize();
        Self::from_nalgebra(&result)

    }

    fn to_nalgebra(&self) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(self.x, self.y, self.z)
    }

    fn from_nalgebra(vector: &nalgebra::Vector3<f32>) -> Self {
        Self::new(vector[0], vector[1], vector[2])
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
