use crate::Vector3f;

use super::primitive::Primitive;

pub struct Scene {
    pub sky: Sky,
    pub primitives: Vec<Primitive>,
    pub transforms: Vec<Transform>,
    pub materials: Vec<Material>,
}

pub struct Sky {
    pub lower_color: Vector3f,
    pub upper_color: Vector3f,
}

pub struct Transform {
    pub translation: Vector3f,
    pub rotation: Vector3f,
    pub scale: Vector3f,
}

pub struct Material {
    pub albedo: Vector3f,
    pub emission: Vector3f,
}
