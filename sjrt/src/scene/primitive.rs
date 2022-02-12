pub enum Primitive {
    Sphere(SphereData),
    Box(BoxData),
}

pub struct SphereData {
    pub radius: f32,
}

pub struct BoxData {
    pub width: f32,
    pub height: f32,
}
