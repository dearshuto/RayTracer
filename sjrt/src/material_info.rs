use crate::Property;

#[derive(Debug, Copy, Clone)]
pub enum Brdf {
    Lambert,
    PerfectSpecularReflection,
}

pub struct MaterialInfo {
    pub normal: nalgebra::Vector3<f32>,
    pub position: nalgebra::Vector3<f32>,
    pub property: Property<f32, nalgebra::Vector3<f32>>,
}

impl MaterialInfo {
    pub fn new(
        normal: nalgebra::Vector3<f32>,
        position: nalgebra::Vector3<f32>,
        property: Property<f32, nalgebra::Vector3<f32>>,
    ) -> Self {
        Self {
            normal,
            position,
            property,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MaterialId {
    Lambert,
    Glass,
}
