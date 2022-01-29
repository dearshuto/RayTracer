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

    pub fn to_nalgebra(&self) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(self.x, self.y, self.z)
    }
}
