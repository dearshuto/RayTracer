pub enum Primitive<TFloat: num::Float> {
    Sphere(SphereData<TFloat>),
    Box(BoxData<TFloat>),
    TriMesh(TriMeshData<TFloat>),
}

pub struct SphereData<TFloat: num::Float> {
    pub radius: TFloat,
}

pub struct BoxData<TFloat: num::Float> {
    pub width: TFloat,
    pub height: TFloat,
    pub depth: TFloat,
}

pub struct TriMeshData<TFloat: num::Float> {
    pub positions: Vec<TFloat>,
    pub indices: Vec<u32>,
}
