use std::ops::Range;

use crate::Vector3f;

pub struct Camera {
    #[allow(dead_code)]
    field_of_view: f32,

    resolution_x: u32,
    resolution_y: u32,
    pub position: Vector3f,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            field_of_view: 0.0,
            resolution_x: width,
            resolution_y: height,
            position: Vector3f::new(2.780, 2.730, -8.000),
        }
    }

    pub fn calculate_ray_direction(&self) -> Vec<RayInfo> {
        self.calculate_ray_direction_range(0..self.resolution_x, 0..self.resolution_y)
    }

    pub fn calculate_ray_direction_range(&self, width_range: Range<u32>, height_range: Range<u32>) -> Vec<RayInfo> {

        let mut results = Vec::new();
        for y in height_range {
            for x in width_range.clone() {
                let lower_left = Vector3f::new(2.77625, 2.72625, -7.990);
                let stride_width = 0.0075 / (self.resolution_x as f32);
                let stride_height = 0.0075 / (self.resolution_y as f32);

                let camera_position = self.position;
                let local_target = lower_left
                    + Vector3f::new((x as f32) * stride_width, y as f32 * stride_height, 0.0);
                let directioin = (local_target - camera_position).normalize();

                let ray_info = RayInfo{ x, y, directions: vec![directioin]};
                results.push(ray_info);
            }
        }

        results
    }
}

pub struct RayInfo {
    pub x: u32,
    pub y: u32,
    pub directions: Vec<Vector3f>,
}
