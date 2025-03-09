use std::ops::{Add, Mul, Range, Sub};

use crate::{traits::INormalized, IConstract, INorm, IOuterProduct};

pub struct Builder<TPoint> {
    field_of_view: f32,
    position: TPoint,
    look_at: TPoint,
}

impl<TPoint> Builder<TPoint>
where
    TPoint: num::Zero + IConstract<f32>,
{
    fn new() -> Self {
        Self {
            field_of_view: std::f32::consts::PI / 4.0,
            position: TPoint::zero(),
            look_at: TPoint::new(0.0, 0.0, -1.0),
        }
    }
}

impl<TPoint> Builder<TPoint>
where
    TPoint: Copy,
{
    pub fn with_position(mut self, position: &TPoint) -> Self {
        self.position = *position;
        self
    }

    pub fn with_look_at(mut self, look_at: &TPoint) -> Self {
        self.look_at = *look_at;
        self
    }
}

impl<TPoint> Builder<TPoint>
where
    TPoint: Copy
        + num::Zero
        + IConstract<f32>
        + Mul<f32, Output = TPoint>
        + Sub<Output = TPoint>
        + INorm
        + INormalized
        + IOuterProduct,
{
    pub fn build(self) -> Camera<TPoint> {
        let direction = self.look_at - self.position;
        let distance_to_look_at = direction.norm();
        let right_vector = direction.cross(&TPoint::new(0.0, 1.0, 0.0)).normalized();

        let up_vector = right_vector.cross(&direction).normalized();
        let up_distance = distance_to_look_at * (self.field_of_view.tan());
        let top_center = direction + up_vector * up_distance;

        Camera {
            center_top: top_center,
            right_vector,
            up_vector,
            half_height: up_distance,
            position: self.position,
        }
    }

    pub fn with_field_of_view(mut self, fov: f32) -> Self {
        self.field_of_view = fov;
        self
    }
}

pub struct Camera<TVector> {
    center_top: TVector,
    right_vector: TVector,
    up_vector: TVector,
    half_height: f32,
    position: TVector,
}

impl Camera<nalgebra::Vector3<f32>> {
    pub fn builder() -> Builder<nalgebra::Vector3<f32>> {
        Builder::new()
    }
}

impl<TVector> Camera<TVector>
where
    TVector: Copy
        + Add<TVector, Output = TVector>
        + Sub<Output = TVector>
        + Mul<f32, Output = TVector>
        + INormalized,
{
    pub fn position(&self) -> &TVector {
        &self.position
    }

    pub fn calculate_ray_direction(&self) -> Vec<RayInfo<TVector>> {
        self.calculate_ray_direction_range(640, 480, 0..640, 0..480)
    }

    pub fn calculate_ray_direction_range(
        &self,
        width: u32,
        height: u32,
        width_range: Range<u32>,
        height_range: Range<u32>,
    ) -> Vec<RayInfo<TVector>> {
        assert!(width_range.end <= width);
        assert!(height_range.end <= height);

        let mut results = Vec::new();

        let width = width as f32;
        let height = height as f32;
        let aspect = width / height;
        let half_width = self.half_height * aspect;
        let top_left = self.center_top - self.right_vector * half_width;

        // 解像度で分割してレイを飛ばすストライドを算出
        // 原点は左上
        let stride_x = self.right_vector * (2.0 * half_width / width);
        let stride_y = (self.up_vector * (-1.0)) * (2.0 * self.half_height / height);

        for y in height_range {
            for x in width_range.clone() {
                // レイを飛ばす方法を画面左上からのオフセットとして算出
                let offset = stride_x * (x as f32) + stride_y * (y as f32);
                let to = top_left + offset;
                let directioin = (to - self.position).normalized();

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

pub struct RayInfo<T> {
    pub x: u32,
    pub y: u32,
    pub directions: Vec<T>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default() {
        // デフォルトで原点から画角 45 度で  (0.0, 0.0, -1.0) を見ている
        let camera = Camera::builder().build();
        let rays = camera.calculate_ray_direction_range(10, 10, 0..10, 0..10);
        assert_eq!(rays.len(), 100);

        assert_eq!(
            rays[0].directions[0],
            nalgebra::Vector3::new(-1.0, 1.0, -1.0).normalize()
        );

        assert_eq!(
            rays[1].directions[0],
            nalgebra::Vector3::new(-1.0 + 0.2, 1.0, -1.0).normalize()
        );

        assert_eq!(
            rays[10].directions[0],
            nalgebra::Vector3::new(-1.0, 1.0 - 0.2, -1.0).normalize()
        );
        assert_eq!(
            rays[11].directions[0],
            nalgebra::Vector3::new(-1.0 + 0.2, 1.0 - 0.2, -1.0).normalize()
        );
    }

    #[test]
    fn rotate_y() {
        // 原点から画角 45 度で (-1.0, 0.0, 0.0) を見ている
        let camera = Camera::builder()
            .with_position(&nalgebra::Vector3::zeros())
            .with_look_at(&nalgebra::Vector3::new(-1.0, 0.0, 0.0))
            .build();
        let rays = camera.calculate_ray_direction_range(10, 10, 0..10, 0..10);
        assert_eq!(rays.len(), 100);

        assert_eq!(
            rays[0].directions[0],
            nalgebra::Vector3::new(-1.0, 1.0, 1.0).normalize()
        );
    }

    #[test]
    fn aspect() {
        // デフォルトで原点から画角 45 度で  (0.0, 0.0, -1.0) を見ている
        let camera = Camera::builder().build();
        let rays = camera.calculate_ray_direction_range(20, 10, 0..20, 0..10);
        assert_eq!(rays.len(), 200);

        assert_eq!(
            rays[0].directions[0],
            nalgebra::Vector3::new(-2.0, 1.0, -1.0).normalize()
        );
        assert_eq!(
            rays[199].directions[0],
            nalgebra::Vector3::new(2.0, -1.0, -1.0).normalize()
        );
    }
}
