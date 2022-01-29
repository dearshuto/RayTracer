use crate::Vector3f;

pub struct Ray
{
    pub origin: Vector3f,
    pub direction: Vector3f,
    pub depth: u32,
}

impl Ray{
    pub fn new(origin: Vector3f, direction: Vector3f, depth: u32) -> Self {
        Self {
            origin,
            direction,
            depth
        }
    }
}
