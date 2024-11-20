use serde_derive::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug, Default)]
pub struct SceneData {
    #[allow(unused)]
    pub name: String,

    #[serde(default = "SkyData::default")]
    pub sky: SkyData,

    #[serde(rename = "sphere", default = "Vec::new")]
    pub spheres: Vec<SphereData>,

    #[serde(rename = "box", default = "Vec::new")]
    pub boxes: Vec<BoxData>,
}

#[allow(unused)]
#[derive(Deserialize, Debug, Default)]
pub struct SkyData {
    pub lower_color: [f32; 3],

    pub upper_color: [f32; 3],
}

#[allow(unused)]
#[derive(Deserialize, Debug, Default)]
pub struct SphereData {
    pub radius: f32,

    #[serde(default = "TransformData::default")]
    pub transform: TransformData,

    #[serde(default = "MaterialData::default")]
    pub material: MaterialData,
}

#[allow(unused)]
#[derive(Deserialize, Debug, Default)]
pub struct BoxData {
    pub width: f32,

    pub height: f32,

    pub depth: f32,

    #[serde(default = "TransformData::default")]
    pub transform: TransformData,

    #[serde(default = "MaterialData::default")]
    pub material: MaterialData,
}

#[allow(unused)]
#[derive(Deserialize, Debug, Default)]
pub struct TransformData {
    #[serde(default = "Float3Data::default")]
    pub translation: Float3Data,

    #[serde(default = "Float3Data::default")]
    pub rotation: Float3Data,

    #[serde(default = "Float3Data::one")]
    pub scale: Float3Data,
}

impl TransformData {
    pub fn default() -> Self {
        Self {
            translation: Float3Data::zero(),
            rotation: Float3Data::zero(),
            scale: Float3Data::one(),
        }
    }
}

#[allow(unused)]
#[derive(Deserialize, Debug, Default)]
pub struct MaterialData {
    pub albedo: Float3Data,
    pub emission: Float3Data,
}

impl MaterialData {
    pub fn default() -> Self {
        Self {
            albedo: Float3Data::one(),
            emission: Float3Data::zero(),
        }
    }
}

#[allow(unused)]
#[derive(Deserialize, Debug, Default)]
pub struct Float3Data {
    #[serde(default = "Default::default")]
    pub x: f32,

    #[serde(default = "Default::default")]
    pub y: f32,

    #[serde(default = "Default::default")]
    pub z: f32,
}

impl Float3Data {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}
