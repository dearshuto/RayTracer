use image::GenericImage;
use sjrt::Vector3f;

#[derive(Debug, Default)]
struct Payload {
    pub depth: u32,
    pub color: [u8; 4],
}

struct Image(image::DynamicImage);

impl sjrt::IColorBuffer for &mut Image {
    fn write(&mut self, x: u32, y: u32, color: sjrt::Color) {
        let sjrt::Color::R8G8B8A8_Uint(data) = color else {
            return;
        };

        self.0.put_pixel(x, y, image::Rgba::from(data));
    }
}

struct Pipeline;

impl sjrt::IRayTracingPipeline for &mut Pipeline {
    type PayloadType = Payload;
    type HitParams = rapier3d::geometry::RayIntersection;

    fn entry(&self, _entry_params: &sjrt::EntryParams) -> Self::PayloadType {
        Payload::default()
    }

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> sjrt::HitAction<Self::PayloadType, impl Iterator<Item = sjrt::RayParams<Self::PayloadType>>>
    {
        if true {
            let normal = hit_params
                .normal
                .map(|c| (c * 255.0).clamp(0.0, u8::MAX as f32) as u8);

            sjrt::HitAction::Payload(Payload {
                depth: payload.depth,
                color: [normal.x, normal.y, normal.z, u8::MAX],
            })
        } else {
            sjrt::HitAction::RayGenerate([].into_iter())
        }
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        // 背景色
        Payload {
            depth: payload.depth,
            color: [25, 50, 75, u8::MAX],
        }
    }

    fn trace(
        &self,
        ray_params: sjrt::RayParams<Self::PayloadType>,
    ) -> sjrt::TraceAction<Self::PayloadType> {
        // レイの反射は 1 回だけにするので即終了
        sjrt::TraceAction::Finish(ray_params.payload)
    }

    fn write(&self, payload: Self::PayloadType) -> sjrt::Color {
        sjrt::Color::R8G8B8A8_Uint(payload.color)
    }
}

fn main() {
    let scene_data = sjrt::scene::Scene {
        sky: sjrt::scene::Sky {
            lower_color: sjrt::Vector3f::zero(),
            upper_color: sjrt::Vector3f::new(0.2, 0.2, 0.6),
        },
        primitives: vec![sjrt::scene::primitive::Primitive::Sphere(
            sjrt::scene::primitive::SphereData { radius: 5.0f32 },
        )],
        transforms: vec![sjrt::scene::Transform::new_with_translation(
            &Vector3f::new(0.0, 0.0, 10.0),
        )],
        materials: vec![sjrt::scene::Material {
            albedo: Vector3f::new(0.1, 0.2, 1.0),
            emission: Vector3f::new(0.1, 0.1, 0.1),
        }],
    };
    let scene = sjrt::util::RapierScene::new_from_scene(&scene_data);
    let mut pipeline = Pipeline {};
    let mut buffer = Image(image::DynamicImage::new_rgba8(640, 480));
    sjrt::Executor::default().execute(&mut buffer, scene, &mut pipeline);

    buffer.0.save("executor.png").unwrap();
}
