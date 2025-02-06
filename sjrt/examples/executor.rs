use image::GenericImage;
use sjrt::Vector3f;

struct Image(image::DynamicImage);

impl sjrt::IPayloadBuffer<u32, [u8; 4]> for &mut Image {
    fn write(&mut self, id: u32, payload: [u8; 4]) {
        let x = id & 0xFFFF;
        let y = (id >> 16) & 0xFFFF;
        self.0.put_pixel(x, y, image::Rgba::from(payload));
    }
}

struct Pipeline;

impl sjrt::IRayTracingPipeline for &mut Pipeline {
    type PayloadType = [u8; 4];
    type RayId = u32;

    fn generate_rays(&self) -> impl Iterator<Item = (Self::RayId, sjrt::RayParams)> {
        let mut results = Vec::default();

        let from = Vector3f::new(0.0, 0.0, -10.0);
        for y in 0..480 {
            for x in 0..640 {
                let ray = Vector3f::new(-320.0 + x as f32, -240.0 + y as f32, 1000.0) - from;
                let ray_params = sjrt::RayParams {
                    from,
                    to: Vector3f::new(ray.x, ray.y, ray.z),
                };

                // (x, y) を 16bit ずつパッキング
                // ユニークな値なのでそのまま ID として使いつつ、ID から x, y が抽出できるようにする
                let id = (y << 16) | x;
                results.push((id as u32, ray_params));
            }
        }

        results.into_iter()
    }

    fn react_closest_hit(
        &self,
    ) -> sjrt::HitAction<Self::PayloadType, impl Iterator<Item = sjrt::RayParams>> {
        if true {
            sjrt::HitAction::Payload([u8::MAX, u8::MAX, u8::MAX, u8::MAX])
        } else {
            sjrt::HitAction::RayGenerate([].into_iter())
        }
    }

    fn react_hit_miss(&self) -> Self::PayloadType {
        // 背景色
        [25, 50, 75, u8::MAX]
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
