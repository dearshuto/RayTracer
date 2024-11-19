use crate::{Property, Vector3f};

#[derive(Debug, Copy, Clone)]
pub enum Brdf {
    Lambert,
    PerfectSpecularReflection,
}

pub struct MaterialInfo {
    pub normal: Vector3f,
    pub position: Vector3f,
    pub property: Property<f32, Vector3f>,
}

impl MaterialInfo {
    pub fn new(normal: Vector3f, position: Vector3f, property: Property<f32, Vector3f>) -> Self {
        Self {
            normal,
            position,
            property,
        }
    }
}
