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

        let Ok(_result): Result<SceneData, _> = serde_xml_rs::from_reader(reader) else {
            return Err(());
        };

        todo!()
    }
}

pub struct Loader<TDeserializer> {
    deserializer: TDeserializer,
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
