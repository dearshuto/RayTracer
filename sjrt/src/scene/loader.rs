use std::path::Path;

use super::{
    primitive::{BoxData, Primitive, SphereData},
    Scene,
};

pub struct Loader {}

impl Loader {
    pub fn load_from_file<TPath: AsRef<Path>>(path: &TPath) -> Scene {
        let reader = std::io::BufReader::new(std::fs::File::open(path).unwrap());
        let scene: detail::Scene = serde_xml_rs::from_reader(reader).unwrap();
        Self::load_scene_impl(&scene)
    }

    pub fn load_from_text(text: &str) -> Scene {
        let scene: detail::Scene = serde_xml_rs::from_str(text).unwrap();
        Self::load_scene_impl(&scene)
    }

    fn load_scene_impl(scene: &detail::Scene) -> Scene {
        let mut primitives = Vec::new();
        let mut transforms = Vec::new();
        let mut materials = Vec::new();

        for sphere in &scene.spheres {
            let sphere_data = SphereData {
                radius: sphere.radius,
            };
            primitives.push(Primitive::Sphere(sphere_data));
            transforms.push(sphere.transform.to_scene_data());
            materials.push(sphere.material.to_scene_data());
        }

        for box_ in &scene.boxes {
            let box_data = BoxData {
                width: box_.width,
                height: box_.height,
                depth: box_.depth,
            };
            primitives.push(Primitive::Box(box_data));
            transforms.push(detail::TransformData::default().to_scene_data());
            materials.push(box_.material.to_scene_data());
        }

        Scene {
            primitives,
            transforms,
            materials,
            sky: scene.sky.to_scene_data(),
        }
    }
}

mod detail {
    use serde_derive::Deserialize;

    use crate::{
        scene::scene::{Material, Sky, Transform},
        Vector3f,
    };

    #[derive(Deserialize, Debug, Default)]
    pub struct Scene {
        pub name: String,

        #[serde(default = "CameraData::new")]
        pub camera: CameraData,

        #[serde(default = "SkyData::default")]
        pub sky: SkyData,

        #[serde(rename = "sphere", default = "Vec::new")]
        pub spheres: Vec<SphereData>,

        #[serde(rename = "box", default = "Vec::new")]
        pub boxes: Vec<BoxData>,
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct CameraData {
        pub position: Float3Data,

        #[serde(default = "Float3Data::zero")]
        pub look_at: Float3Data,
    }

    impl CameraData {
        pub fn new() -> Self {
            Self {
                position: Float3Data {
                    x: 0.0,
                    y: 0.0,
                    z: -10.0,
                },
                look_at: Float3Data::zero(),
            }
        }
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct SkyData {
        pub lower_color: [f32; 3],

        pub upper_color: [f32; 3],
    }

    impl SkyData {
        pub fn to_scene_data(&self) -> Sky {
            Sky {
                lower_color: Vector3f::new(
                    self.lower_color[0],
                    self.lower_color[1],
                    self.lower_color[2],
                ),
                upper_color: Vector3f::new(
                    self.upper_color[0],
                    self.upper_color[1],
                    self.upper_color[2],
                ),
            }
        }
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
    pub struct BoxData {
        pub width: f32,

        pub height: f32,

        pub depth: f32,

        #[serde(default = "TransformData::default")]
        pub transform: TransformData,

        #[serde(default = "MaterialData::default")]
        pub material: MaterialData,
    }

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

        pub fn to_scene_data(&self) -> Transform {
            Transform {
                translation: Vector3f::new(
                    self.translation.x,
                    self.translation.y,
                    self.translation.z,
                ),
                rotation: Vector3f::new(self.rotation.x, self.rotation.y, self.rotation.z),
                scale: Vector3f::new(self.scale.x, self.scale.y, self.scale.z),
            }
        }
    }

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

        pub fn to_scene_data(&self) -> Material {
            Material {
                albedo: Vector3f::new(self.albedo.x, self.albedo.y, self.albedo.z),
                emission: Vector3f::new(self.emission.x, self.emission.y, self.emission.z),
            }
        }
    }

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
}

#[cfg(test)]
mod tests {
    use super::Loader;

    #[test]
    fn load_empty_scene() {
        let xml = r##"
<scene name="main_body">
</scene>
"##;

        let scene = Loader::load_from_text(xml);
        assert!(scene.primitives.is_empty());
        assert!(scene.transforms.is_empty());
    }

    #[test]
    fn load_simple_sphere_scene() {
        let xml = r##"
<scene name="main_body">
    <sphere radius = "0.5"/>
</scene>
"##;

        let scene = Loader::load_from_text(xml);
        let _sphere = scene.primitives.first().unwrap();
        let transform = scene.transforms.first().unwrap();

        // 位置
        assert_eq!(transform.translation.x, 0.0);
        assert_eq!(transform.translation.y, 0.0);
        assert_eq!(transform.translation.z, 0.0);

        // 回転
        assert_eq!(transform.rotation.x, 0.0);
        assert_eq!(transform.rotation.y, 0.0);
        assert_eq!(transform.rotation.z, 0.0);

        // スケール
        assert_eq!(transform.scale.x, 1.0);
        assert_eq!(transform.scale.y, 1.0);
        assert_eq!(transform.scale.z, 1.0);
    }

    #[test]
    fn load_sphere_transform_scene() {
        let xml = r##"
<scene name="main_body">
    <sphere radius = "0.5">
        <transform>
            <translation x = "1.0" y = "2.0" z = "3.0"/>
            <rotation x = "15.0" y = "30.0" z = "45.0"/>
            <scale x = "4.0" y = "5.0" z = "6.0"/>
        </transform>
    </sphere>
</scene>
"##;

        let scene = Loader::load_from_text(xml);
        let transform = scene.transforms.first().unwrap();

        // 位置
        assert_eq!(transform.translation.x, 1.0);
        assert_eq!(transform.translation.y, 2.0);
        assert_eq!(transform.translation.z, 3.0);

        // 回転
        assert_eq!(transform.rotation.x, 15.0);
        assert_eq!(transform.rotation.y, 30.0);
        assert_eq!(transform.rotation.z, 45.0);

        // スケール
        assert_eq!(transform.scale.x, 4.0);
        assert_eq!(transform.scale.y, 5.0);
        assert_eq!(transform.scale.z, 6.0);
    }

    #[test]
    fn load_partial_transform_scene() {
        let xml = r##"
<scene name="main_body">
    <sphere radius = "0.5">
        <transform>
            <translation x = "2.0" y = "3.0"/>
            <rotation z = "120.0"/>
        </transform>
    </sphere>
</scene>
"##;

        let scene = Loader::load_from_text(xml);
        let transform = scene.transforms.first().unwrap();
        match scene.primitives.first().unwrap() {
            crate::scene::primitive::Primitive::Sphere(sphere) => assert_eq!(sphere.radius, 0.5),
            _ => panic!(),
        }

        // 位置
        assert_eq!(transform.translation.x, 2.0);
        assert_eq!(transform.translation.y, 3.0);
        assert_eq!(transform.translation.z, 0.0);

        // 回転
        assert_eq!(transform.rotation.x, 0.0);
        assert_eq!(transform.rotation.y, 0.0);
        assert_eq!(transform.rotation.z, 120.0);

        // スケール
        assert_eq!(transform.scale.x, 1.0);
        assert_eq!(transform.scale.y, 1.0);
        assert_eq!(transform.scale.z, 1.0);
    }
}
