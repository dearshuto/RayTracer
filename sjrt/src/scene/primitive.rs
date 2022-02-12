pub enum Primitive {
    Sphere(SphereData),
    Box(BoxData),
    TriMesh(TriMeshData),
}

pub struct SphereData {
    pub radius: f32,
}

pub struct BoxData {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

pub struct TriMeshData {
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
}
