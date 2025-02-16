use std::sync::Arc;

#[tokio::main]
async fn main() {
    let scene_data = sjrt::scene::Scene::box_point_light();
    let scene = Arc::new(sjrt::util::RapierScene::new_from_scene(&scene_data));
    let pipeline = Arc::new(sjrt::PathTracerEx::default());
    let mut buffer = sjrt::util::ImageBuffer::new(640, 480);
    sjrt::Executor::default()
        .execute_async(&mut buffer, scene, pipeline)
        .await;

    buffer.save("executor.png");
}
