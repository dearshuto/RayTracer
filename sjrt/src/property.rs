use crate::Brdf;

#[derive(Debug, Copy, Clone)]
pub struct Property
{
    pub metaric: f32,
    pub roughness: f32,
    pub emission: f32,
    pub brdf: Brdf,
}

impl Property{
    pub fn new(metaric: f32, roughness: f32, emission: f32, brdf: Brdf) -> Self {
        Self {
            metaric,
            roughness,
            emission,
            brdf
        }
    }
}
