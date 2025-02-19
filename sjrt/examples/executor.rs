use std::sync::Arc;

#[tokio::main]
async fn main() {
    let scene_data = sjrt::scene::Scene::box_point_light();
    let scene = Arc::new(sjrt::util::RapierScene::new_from_scene(&scene_data));
    let pipeline = Arc::new(
        sjrt::PathTracerEx::default()
            .with_depth(4)
            .with_sampling_count(64),
    );

    let mut buffer = sjrt::util::ImageBuffer::new(640, 480);
    let rays = sjrt::Camera::builder()
        .with_position(&nalgebra::Vector3::new(0.0, 7.0, 20.0))
        .with_look_at(&nalgebra::Vector3::new(0.0, 5.0, 0.0))
        .with_field_of_view(std::f32::consts::PI / 4.0)
        .build()
        .calculate_ray_direction();
    sjrt::Executor::default()
        .execute_async(&mut buffer, rays.into_iter(), scene, pipeline)
        .await;

    buffer.save("executor.png");
}
