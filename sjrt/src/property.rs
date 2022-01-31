use crate::{Brdf, Vector3f};

#[derive(Debug, Copy, Clone)]
pub struct Property
{
    pub metaric: f32,
    pub roughness: f32,
    pub emission: f32,
    pub brdf: Brdf,
    pub albedo: Vector3f,
}

impl Property{
    pub fn new(metaric: f32, roughness: f32, emission: f32, brdf: Brdf, albedo: Vector3f) -> Self {
        Self {
            metaric,
            roughness,
            emission,
            brdf,
            albedo: albedo,
        }
    }
}
