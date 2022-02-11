use crate::{Brdf, Vector3f};

#[derive(Debug, Copy, Clone)]
pub struct Property
{
    pub metaric: f32,
    pub roughness: f32,
    pub emission: Vector3f,
    pub diffuse_brdf: Brdf,
    pub specular_brdf: Brdf,
    pub albedo: Vector3f,
}

impl Default for Property {
    fn default() -> Self {
        Self {
            metaric: 0.2,
            roughness: Default::default(),
            emission: Vector3f::zero(),
            diffuse_brdf: Brdf::Lambert,
            specular_brdf: Brdf::PerfectSpecularReflection,
            albedo: Vector3f::new(1.0, 1.0, 1.0)
        }
    }
}
