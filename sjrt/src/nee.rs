use rand::prelude::*;
use crate::{Vector3f, IScene};

pub struct NextEventEstimation {}

impl NextEventEstimation {
    pub fn new() -> Self {
        Self{}
    }

    pub fn estimate<TScene: IScene>(&self, position: &Vector3f, normal: &Vector3f, scene: &TScene) -> Vec<Vector3f> {
        let result =  scene.enumerate_related_lights(position);

        // 光源に向かう方向を重点的にサンプル
        let direction_candidate : Vec<Vector3f> = result.centers.iter().map(|light_center| { (*light_center - *position).normalize() } ).collect();
        let mut rng = thread_rng();
        let n : rand_distr::Normal<f32> = rand_distr::Normal::new(0.0, 0.5).unwrap();
        let distribution_x: f32 = n.sample(&mut rng).clamp(-1.0, 1.0);
        let distribution_y: f32 = n.sample(&mut rng).clamp(-1.0, 1.0);
        let distribution_z: f32 = n.sample(&mut rng).clamp(-1.0, 1.0);

        // 暫定的に光源は一個だと仮定
        let result = (direction_candidate[0] + Vector3f::new(distribution_x, distribution_y, distribution_z)).normalize();

        // 物体のウラ面にレイが飛んでたら修正
        // TODO: 透過対応
        let modified_result = if 0.0 < result.dot(normal) {
            result
        } else {
            result + 2.0 * (-result.dot(normal)) * *normal
        };
        vec![modified_result]
    }
}
