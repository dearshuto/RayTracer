use sjrt::IScene;

use crate::{Float3, MaterialInfoData};

pub struct ExternalScene {
    cast_ray_callback: unsafe extern "C" fn(
        out_material_info_data: *mut MaterialInfoData,
        from: *const Float3,
        to: *const Float3,
    ) -> bool,
}

impl ExternalScene {
    pub fn new(
        cast_ray_callback: unsafe extern "C" fn(
            out_material_info_data: *mut MaterialInfoData,
            from: *const Float3,
            to: *const Float3,
        ) -> bool,
    ) -> Self {
        Self { cast_ray_callback }
    }
}

impl IScene for ExternalScene {
    fn cast_ray(&self, from: &sjrt::Vector3f, to: &sjrt::Vector3f) -> Option<sjrt::MaterialInfo> {
        // コールバック経由でデータを取得
        let from_data = Float3::from_vector(from);
        let to_data = Float3::from_vector(to);
        let mut material_info_data = MaterialInfoData {
            normal: Float3::zero(),
            position: Float3::zero(),
        };

        // sjrt の型に変換
        if unsafe { (self.cast_ray_callback)(&mut material_info_data, &from_data, &to_data) } {
            let material_info = material_info_data.to_material_info();
            Some(material_info)
        } else {
            None
        }
    }

    fn enumerate_related_lights(&self, _position: &sjrt::Vector3f) -> sjrt::EnumerateLightResult {
        todo!()
    }

    fn find_background_color(
        &self,
        _position: &sjrt::Vector3f,
        _direction: &sjrt::Vector3f,
    ) -> sjrt::Vector3f {
        // とりあえず決め打ち
        sjrt::Vector3f::new(0.0, 0.2, 0.3)
    }
}
