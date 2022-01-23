#[derive(Debug, Copy, Clone)]
pub struct Property
{
    pub metaric: f32,
    pub roughness: f32,
    pub emission: f32,
}

impl Property{
    pub fn new(metaric: f32, roughness: f32, emission: f32) -> Self {
        Self {
            metaric,
            roughness,
            emission,
        }
    }
}
