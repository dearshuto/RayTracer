use sjrt::scene::{primitive::SphereData, Material, Transform};
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsCast};
use web_sys::{HtmlCanvasElement, ImageData};

struct Buffer {
    width: i32,
    height: i32,
    internal: Vec<u8>,
}

impl Buffer {
    pub fn new(width: i32, height: i32) -> Self {
        let mut internal = Vec::default();
        internal.resize_with((4 * width * height) as usize, Default::default);

        Self {
            width,
            height,
            internal,
        }
    }
}

impl sjrt::IBuffer for Buffer {
    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }

    fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8) {
        let index = (x + self.width * y) as usize;
        self.internal[4 * index] = red;
        self.internal[4 * index + 1] = green;
        self.internal[4 * index + 2] = blue;
        self.internal[4 * index + 3] = u8::MAX;
    }
}

#[wasm_bindgen]
pub fn render(
    canvas: HtmlCanvasElement,
    width: i32,
    height: i32,
    sampling_count: u16,
    depth_max: u16,
) {
    // let scene = sjrt::scene::Scene {
    //     sky: sjrt::scene::Sky {
    //         lower_color: sjrt::Vector3f::new(0.0, 0.0, 0.0),
    //         upper_color: sjrt::Vector3f::new(1.0, 0.2, 0.3),
    //     },
    //     primitives: vec![sjrt::scene::primitive::Primitive::Sphere(SphereData {
    //         radius: 2.5,
    //     })],
    //     transforms: vec![Transform::new_with_translation(&sjrt::Vector3f::new(
    //         0.0, 0.0, 0.0,
    //     ))],
    //     materials: vec![Material {
    //         albedo: sjrt::Vector3f::new(1.0, 1.0, 1.0),
    //         emission: sjrt::Vector3f::new(1.0, 1.0, 1.0),
    //     }],
    // };
    // let scene = sjrt::util::RapierScene::new_from_scene(&scene);
    let scene = sjrt::util::RapierScene::new();
    let renderer = sjrt::PathTracer::new(sampling_count, depth_max, false);
    let system = sjrt::System::new();
    let mut buffer = Buffer::new(width, height);
    system.execute(&scene, &mut buffer, &renderer);

    let image =
        ImageData::new_with_u8_clamped_array(Clamped(&buffer.internal), width as u32).unwrap();

    let Ok(context_opt) = canvas.get_context("2d") else {
        return;
    };

    let Some(context) = context_opt else {
        return;
    };

    let context = context
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    context.put_image_data(&image, 0.0, 0.0).unwrap();
}
