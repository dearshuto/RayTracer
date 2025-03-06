use serde::Deserialize;
use sjrt::scene::{Material, Scene, Sky, Transform, primitive::Primitive};

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

impl Into<Scene> for SceneData {
    fn into(self) -> Scene {
        let mut primitives = Vec::new();
        let mut transforms = Vec::new();
        let mut materials = Vec::new();

        for sphere in self.spheres.into_iter() {
            let sphere_data = sjrt::scene::primitive::SphereData {
                radius: sphere.radius,
            };
            primitives.push(Primitive::Sphere(sphere_data));
            transforms.push(sphere.transform.into());
            materials.push(sphere.material.into());
        }

        for box_ in self.boxes.into_iter() {
            let box_data = sjrt::scene::primitive::BoxData {
                width: box_.width,
                height: box_.height,
                depth: box_.depth,
            };
            primitives.push(Primitive::Box(box_data));
            transforms.push(box_.transform.into());
            materials.push(box_.material.into());
        }

        Scene {
            sky: self.sky.into(),
            primitives,
            transforms,
            materials,
        }
    }
}

#[allow(unused)]
#[derive(Deserialize, Debug, Default)]
pub struct SkyData {
    pub lower_color: [f32; 3],

    pub upper_color: [f32; 3],
}

impl Into<Sky> for SkyData {
    fn into(self) -> Sky {
        Sky {
            lower_color: nalgebra::Vector3::new(
                self.lower_color[0],
                self.lower_color[1],
                self.lower_color[2],
            ),
            upper_color: nalgebra::Vector3::new(
                self.upper_color[0],
                self.upper_color[1],
                self.upper_color[2],
            ),
        }
    }
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

impl Into<Transform> for TransformData {
    fn into(self) -> Transform {
        Transform {
            translation: nalgebra::Vector3::new(
                self.translation.x,
                self.translation.y,
                self.translation.z,
            ),
            rotation: nalgebra::Vector3::new(self.rotation.x, self.rotation.y, self.rotation.z),
            scale: nalgebra::Vector3::new(self.scale.x, self.scale.y, self.scale.z),
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

impl Into<Material> for MaterialData {
    fn into(self) -> Material {
        Material {
            albedo: nalgebra::Vector3::new(self.albedo.x, self.albedo.y, self.albedo.z),
            emission: nalgebra::Vector3::new(self.emission.x, self.emission.y, self.emission.z),
            specular: 0.0,
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
