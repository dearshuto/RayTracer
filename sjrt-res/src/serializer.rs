use std::{
    io::{BufReader, Read},
    path::Path,
};

use sjrt::scene::Scene;

use crate::detail::SceneData;

pub trait IDeserializer {
    type Error;

    fn deserialize<TRead: Read>(&self, read: TRead) -> Result<Scene, Self::Error>;
}

// Xml としてデシリアライズ
pub struct XmlDeserialize;
impl IDeserializer for XmlDeserialize {
    type Error = ();

    fn deserialize<TRead: Read>(&self, read: TRead) -> Result<Scene, ()> {
        let reader = BufReader::new(read);

        let Ok(result): Result<SceneData, _> = serde_xml_rs::from_reader(reader) else {
            return Err(());
        };

        Ok(result.into())
    }
}

pub struct Loader<TDeserializer> {
    deserializer: TDeserializer,
}

impl Loader<XmlDeserialize> {
    pub fn xml() -> Self {
        Self {
            deserializer: XmlDeserialize,
        }
    }
}

impl<TDeserializer> Loader<TDeserializer>
where
    TDeserializer: IDeserializer,
{
    #[allow(unused)]
    pub fn deserialize<TRead: Read>(&self, read: TRead) -> Result<Scene, ()> {
        let Ok(result) = self.deserializer.deserialize(read) else {
            return Err(());
        };

        Ok(result)
    }

    #[allow(unused)]
    pub fn deserialize_from<TPath: AsRef<Path>>(&self, path: &TPath) -> Result<Scene, ()> {
        let Ok(file) = std::fs::File::open(path) else {
            return Err(());
        };

        let Ok(scene) = self.deserialize(file) else {
            return Err(());
        };

        Ok(scene)
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

        let scene = Loader::xml().deserialize(xml.as_bytes()).unwrap();
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

        let scene = Loader::xml().deserialize(xml.as_bytes()).unwrap();
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

        let scene = Loader::xml().deserialize(xml.as_bytes()).unwrap();
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

        let scene = Loader::xml().deserialize(xml.as_bytes()).unwrap();
        let transform = scene.transforms.first().unwrap();
        match scene.primitives.first().unwrap() {
            sjrt::scene::primitive::Primitive::Sphere(sphere) => assert_eq!(sphere.radius, 0.5),
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
