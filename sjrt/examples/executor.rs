use image::GenericImage;
use sjrt::Vector3f;

#[derive(Debug, Default)]
struct Payload {
    pub id: u32,
    pub depth: u32,
    pub color: [u8; 4],
}

struct Image(image::DynamicImage);

impl sjrt::IPayloadBuffer<Payload> for &mut Image {
    fn write(&mut self, payload: Payload) {
        let id = payload.id;
        let x = id & 0xFFFF;
        let y = (id >> 16) & 0xFFFF;
        self.0.put_pixel(x, y, image::Rgba::from(payload.color));
    }
}

struct Pipeline;

impl sjrt::IRayTracingPipeline for &mut Pipeline {
    type PayloadType = Payload;
    type HitParams = rapier3d::geometry::RayIntersection;

    fn entry(&self) -> impl Iterator<Item = Self::PayloadType> {
        let mut results = Vec::default();

        for y in 0..480 {
            for x in 0..640 {
                // (x, y) を 16bit ずつパッキング
                // ユニークな値なのでそのまま ID として使いつつ、ID から x, y が抽出できるようにする
                let id = (y << 16) | x;
                results.push(Payload {
                    id,
                    ..Default::default()
                });
            }
        }

        results.into_iter()
    }

    fn trace(&self, payload: Self::PayloadType) -> sjrt::TraceAction<Self::PayloadType> {
        if 0 < payload.depth {
            return sjrt::TraceAction::Finish(payload);
        }

        let id = payload.id;
        let x = id & 0xFFFF;
        let y = (id >> 16) & 0xFFFF;

        let from = Vector3f::new(0.0, 0.0, -10.0);
        let ray = Vector3f::new(-320.0 + x as f32, -240.0 + y as f32, 1000.0) - from;
        let ray_params = sjrt::RayParams {
            from,
            to: Vector3f::new(ray.x, ray.y, ray.z),
            payload: Payload {
                id,
                depth: 1,
                ..Default::default()
            },
        };

        sjrt::TraceAction::Next(ray_params)
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
                id: payload.id,
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
            id: payload.id,
            depth: payload.depth,
            color: [25, 50, 75, u8::MAX],
        }
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
