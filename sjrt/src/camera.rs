use std::ops::Range;

use crate::Vector3f;

pub struct Builder {
    resolution_x: u32,
    resolution_y: u32,
    position: Vector3f,
}

impl Builder {
    fn new() -> Self {
        Self {
            resolution_x: 128,
            resolution_y: 128,
            position: Vector3f::zero(),
        }
    }

    pub fn build(self) -> Camera {
        Camera {
            field_of_view: 0.0,
            resolution_x: self.resolution_x,
            resolution_y: self.resolution_y,
            position: self.position,
        }
    }

    pub fn with_position(mut self, position: &Vector3f) -> Self {
        self.position = *position;
        self
    }

    pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
        self.resolution_x = width;
        self.resolution_y = height;
        self
    }
}

pub struct Camera {
    #[allow(dead_code)]
    field_of_view: f32,

    resolution_x: u32,
    resolution_y: u32,
    position: Vector3f,
}

impl Camera {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn position(&self) -> &Vector3f {
        &self.position
    }

    pub fn calculate_ray_direction(&self) -> Vec<RayInfo> {
        self.calculate_ray_direction_range(0..self.resolution_x, 0..self.resolution_y)
    }

    pub fn calculate_ray_direction_range(
        &self,
        width_range: Range<u32>,
        height_range: Range<u32>,
    ) -> Vec<RayInfo> {
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

                let ray_info = RayInfo {
                    x,
                    y,
                    directions: vec![directioin],
                };
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
