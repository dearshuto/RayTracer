use std::path::Path;

use super::{
    primitive::{Primitive, SphereData},
    Scene,
};

pub struct Loader {}

impl Loader {
    pub fn load_from_file<TPath: AsRef<Path>>(_path: &TPath) -> Scene {
        todo!()
    }

    pub fn load_from_text(text: &str) -> Scene {
        let scene: detail::Scene = serde_xml_rs::from_str(text).unwrap();
        let mut primitives = Vec::new();
        let mut transforms = Vec::new();
        let mut materials = Vec::new();

        for sphere in scene.spheres {
            let sphere_data = SphereData {
                radius: sphere.radius,
            };
            primitives.push(Primitive::Sphere(sphere_data));
            transforms.push(sphere.transform.to_scene_data());
            materials.push(sphere.material.to_scene_data());
        }

        Scene {
            primitives,
            transforms,
            materials,
        }
    }
}

mod detail {
    use serde_derive::Deserialize;

    use crate::{
        scene::scene::{Material, Transform},
        Vector3f,
    };

    #[derive(Deserialize, Debug, Default)]
    pub struct Scene {
        pub name: String,

        #[serde(rename = "sphere")]
        pub spheres: Vec<SphereData>,
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct SphereData {
        pub radius: f32,

        #[serde(default = "TransformData::default")]
        pub transform: TransformData,

        #[serde(default = "MaterialData::default")]
        pub material: MaterialData,
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct TransformData {
        pub translation: [f32; 3],

        pub rotation: [f32; 3],

        pub scale: [f32; 3],
    }

    impl TransformData {
        pub fn default() -> Self {
            Self {
                translation: [0.0; 3],
                rotation: [0.0; 3],
                scale: [1.0, 1.0, 1.0],
            }
        }

        pub fn to_scene_data(&self) -> Transform {
            Transform {
                translation: Vector3f::new(
                    self.translation[0],
                    self.translation[1],
                    self.translation[2],
                ),
                rotation: Vector3f::new(self.rotation[0], self.rotation[1], self.rotation[2]),
                scale: Vector3f::new(self.scale[0], self.scale[1], self.scale[2]),
            }
        }
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct MaterialData {
        pub albedo: [f32; 3],
        pub emission: [f32; 3],
    }

    impl MaterialData {
        pub fn default() -> Self {
            Self {
                albedo: [1.0, 1.0, 1.0],
                emission: [0.0, 0.0, 0.0],
            }
        }

        pub fn to_scene_data(&self) -> Material {
            Material {
                albedo: Vector3f::new(self.albedo[0], self.albedo[1], self.albedo[2]),
                emission: Vector3f::new(self.emission[0], self.emission[1], self.emission[2]),
            }
        }
    }
}
