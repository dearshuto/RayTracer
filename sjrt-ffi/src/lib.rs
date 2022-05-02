mod external_buffer;
mod external_scene;
pub use external_buffer::ExternalBuffer;
pub use external_scene::ExternalScene;

#[repr(C)]
pub struct Float3 {
    x: f32,
    y: f32,
    z: f32,
    _padding: f32,
}

impl Float3 {
    pub fn zero() -> Self {
        Float3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            _padding: 0.0,
        }
    }

    pub fn from_vector(vector: &sjrt::Vector3f) -> Self {
        Float3 {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            _padding: 0.0,
        }
    }

    pub fn to_vector(&self) -> sjrt::Vector3f {
        sjrt::Vector3f {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[repr(C)]
pub struct MaterialInfoData {
    normal: Float3,
    position: Float3,
}

impl MaterialInfoData {
    pub fn to_material_info(&self) -> sjrt::MaterialInfo {
        let normal = self.normal.to_vector();
        let position = self.position.to_vector();
        let property = sjrt::Property {
            albedo: sjrt::Vector3f::new(1.0, 0.0, 0.0),
            ..Default::default()
        };
        sjrt::MaterialInfo::new(normal, position, property)
    }
}

#[no_mangle]
pub extern "C" fn create_path_tracer(sampling_count: u16, depth_max: u16) -> *mut sjrt::PathTracer {
    let instance = sjrt::PathTracer::new(sampling_count, depth_max, false /*is_nee_enabled*/);
    Box::into_raw(Box::new(instance))
}

#[no_mangle]
pub extern "C" fn destroy_path_tracer(ptr: *mut sjrt::PathTracer) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn create_default_system() -> *mut sjrt::System {
    let instance = sjrt::System::new();
    Box::into_raw(Box::new(instance))
}

#[no_mangle]
pub extern "C" fn destroy_default_system(ptr: *mut sjrt::System) {
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn render(
    system_ptr: *const sjrt::System,
    scene_ptr: *mut sjrt::RapierScene,
    buffer_ptr: *mut sjrt::image::ImageBuffer,
) {
    let renderer = sjrt::PathTracer::new(64, 1, false);
    unsafe { (*system_ptr).execute(&mut *scene_ptr, &mut *buffer_ptr, &renderer) };
}

#[no_mangle]
pub extern "C" fn render_to_external_buffer(
    system_ptr: *const sjrt::System,
    scene_ptr: *mut sjrt::RapierScene,
    buffer_ptr: *mut ExternalBuffer,
) {
    let renderer = sjrt::PathTracer::new(64, 1, false);
    unsafe { (*system_ptr).execute(&mut *scene_ptr, &mut *buffer_ptr, &renderer) };
}

#[no_mangle]
pub extern "C" fn render_with_external_resource(
    system_ptr: *const sjrt::System,
    scene_ptr: *mut ExternalScene,
    buffer_ptr: *mut ExternalBuffer,
) {
    let renderer = sjrt::PathTracer::new(64, 1, false);
    unsafe { (*system_ptr).execute(&mut *scene_ptr, &mut *buffer_ptr, &renderer) };
}

#[no_mangle]
pub extern "C" fn create_default_scene() -> *mut sjrt::RapierScene {
    let cornel_box = sjrt::scene::Scene::create_cornell_box();
    let instance = sjrt::RapierScene::new_from_scene(&cornel_box);
    Box::into_raw(Box::new(instance))
}

#[no_mangle]
pub extern "C" fn destroy_default_scene(ptr: *mut sjrt::RapierScene) {
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn create_default_buffer(width: i32, height: i32) -> *mut sjrt::image::ImageBuffer {
    let instance = sjrt::image::ImageBuffer::new(width, height);
    Box::into_raw(Box::new(instance))
}

#[no_mangle]
pub extern "C" fn destroy_default_buffer(ptr: *mut sjrt::image::ImageBuffer) {
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn create_external_buffer(
    get_width_callback: unsafe extern "C" fn() -> i32,
    get_height_callback: unsafe extern "C" fn() -> i32,
    set_color_callback: unsafe extern "C" fn(x: i32, y: i32, red: u8, green: u8, blue: u8),
) -> *mut ExternalBuffer {
    let instance = ExternalBuffer::new(get_width_callback, get_height_callback, set_color_callback);
    Box::into_raw(Box::new(instance))
}

#[no_mangle]
pub extern "C" fn destroy_external_buffer(ptr: *mut ExternalBuffer) {
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn create_external_scene(
    cast_ray_callback: unsafe extern "C" fn(
        out_material_info_data: *mut MaterialInfoData,
        from: *const Float3,
        to: *const Float3,
    ) -> bool,
) -> *mut ExternalScene {
    let instance = ExternalScene::new(cast_ray_callback);
    Box::into_raw(Box::new(instance))
}

#[no_mangle]
pub extern "C" fn destroy_external_scene(ptr: *mut ExternalScene) {
    unsafe {
        Box::from_raw(ptr);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
