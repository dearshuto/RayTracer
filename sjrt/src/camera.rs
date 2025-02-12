use std::ops::Range;

use crate::Vector3f;

pub struct Builder {
    field_of_view: f32,
    position: Vector3f,
    look_at: Vector3f,
}

impl Builder {
    fn new() -> Self {
        Self {
            field_of_view: std::f32::consts::PI / 4.0,
            position: Vector3f::zero(),
            look_at: Vector3f::new(0.0, 0.0, -1.0),
        }
    }

    pub fn build(self) -> Camera {
        let direction = self.look_at - self.position;
        let distance_to_look_at = direction.norm();
        let direction = nalgebra::Vector3::new(direction.x, direction.y, direction.z);
        let right_vector = direction
            .cross(&nalgebra::Vector3::new(0.0, 1.0, 0.0))
            .normalize();

        let up_vector = right_vector.cross(&direction).normalize();
        let up_distance = distance_to_look_at * (self.field_of_view.tan());
        let top_center = direction + up_distance * up_vector;

        Camera {
            center_top: Vector3f::new(top_center.x, top_center.y, top_center.z),
            right_vector: Vector3f::new(right_vector.x, right_vector.y, right_vector.z),
            up_vector: Vector3f::new(up_vector.x, up_vector.y, up_vector.z),
            half_height: up_distance,
            position: self.position,
        }
    }

    pub fn with_field_of_view(mut self, fov: f32) -> Self {
        self.field_of_view = fov;
        self
    }

    pub fn with_position(mut self, position: &Vector3f) -> Self {
        self.position = *position;
        self
    }

    pub fn with_look_at(mut self, look_at: &Vector3f) -> Self {
        self.look_at = *look_at;
        self
    }
}

pub struct Camera {
    center_top: Vector3f,
    right_vector: Vector3f,
    up_vector: Vector3f,
    half_height: f32,
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
        self.calculate_ray_direction_range(640, 480, 0..640, 0..480)
    }

    pub fn calculate_ray_direction_range(
        &self,
        width: u32,
        height: u32,
        width_range: Range<u32>,
        height_range: Range<u32>,
    ) -> Vec<RayInfo> {
        assert!(width_range.end <= width);
        assert!(height_range.end <= height);

        let mut results = Vec::new();

        let width = width as f32;
        let height = height as f32;
        let aspect = width / height;
        let half_width = self.half_height * aspect;
        let top_left = self.center_top - half_width * self.right_vector;

        // 解像度で分割してレイを飛ばすストライドを算出
        // 原点は左上
        let stride_x = (2.0 * half_width / width) * self.right_vector;
        let stride_y = (2.0 * self.half_height / height) * (-1.0 * self.up_vector);

        for y in height_range {
            for x in width_range.clone() {
                // レイを飛ばす方法を画面左上からのオフセットとして算出
                let offset = (x as f32) * stride_x + (y as f32) * stride_y;
                let to = top_left + offset;
                let directioin = (to - self.position).normalize();

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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default() {
        // デフォルトで原点から画角 45 度で  (0.0, 0.0, -1.0) を見ている
        let camera = Camera::builder().build();
        let rays = camera.calculate_ray_direction_range(0..10, 0..10);
        assert_eq!(rays.len(), 100);

        assert_eq!(
            rays[0].directions[0],
            Vector3f::new(-1.0, 1.0, -1.0).normalize()
        );

        assert_eq!(
            rays[1].directions[0],
            Vector3f::new(-1.0 + 0.2, 1.0, -1.0).normalize()
        );

        assert_eq!(
            rays[10].directions[0],
            Vector3f::new(-1.0, 1.0 - 0.2, -1.0).normalize()
        );
        assert_eq!(
            rays[11].directions[0],
            Vector3f::new(-1.0 + 0.2, 1.0 - 0.2, -1.0).normalize()
        );
    }

    #[test]
    fn rotate_y() {
        // 原点から画角 45 度で (-1.0, 0.0, 0.0) を見ている
        let camera = Camera::builder()
            .with_position(&Vector3f::zero())
            .with_look_at(&Vector3f::new(-1.0, 0.0, 0.0))
            .build();
        let rays = camera.calculate_ray_direction_range(0..10, 0..10);
        assert_eq!(rays.len(), 100);

        assert_eq!(
            rays[0].directions[0],
            Vector3f::new(-1.0, 1.0, 1.0).normalize()
        );
    }
}
