use std::ops::Range;

use crate::traits::{IVector3, IVectorComponent3};

pub struct Builder<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
{
    resolution_x: u32,
    resolution_y: u32,
    position: TVector3,
    _marker: std::marker::PhantomData<TFloat>,
}

impl<TFloat, TVector3> Builder<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat> + Copy,
{
    fn new() -> Self {
        Self {
            resolution_x: 128,
            resolution_y: 128,
            position: TVector3::zero(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(self) -> Camera<TFloat, TVector3> {
        Camera {
            field_of_view: 0.0,
            resolution_x: self.resolution_x,
            resolution_y: self.resolution_y,
            position: self.position,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_position(mut self, position: &TVector3) -> Self {
        self.position = *position;
        self
    }

    pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
        self.resolution_x = width;
        self.resolution_y = height;
        self
    }
}

pub struct Camera<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
{
    #[allow(dead_code)]
    field_of_view: f32,

    resolution_x: u32,
    resolution_y: u32,
    position: TVector3,

    _marker: std::marker::PhantomData<TFloat>,
}

impl<TFloat, TVector3> Camera<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat> + IVectorComponent3<TFloat> + Copy,
{
    pub fn builder() -> Builder<TFloat, TVector3> {
        Builder::new()
    }

    pub fn position(&self) -> &TVector3 {
        &self.position
    }

    pub fn calculate_ray_direction(&self) -> Vec<RayInfo<TFloat, TVector3>> {
        self.calculate_ray_direction_range(0..self.resolution_x, 0..self.resolution_y)
    }

    pub fn calculate_ray_direction_range(
        &self,
        width_range: Range<u32>,
        height_range: Range<u32>,
    ) -> Vec<RayInfo<TFloat, TVector3>> {
        let mut results = Vec::new();
        for y in height_range {
            for x in width_range.clone() {
                let lower_left = TVector3::new(2.77625, 2.72625, -7.990);
                let stride_width = 0.0075 / (self.resolution_x as f32);
                let stride_height = 0.0075 / (self.resolution_y as f32);

                let camera_position = self.position;
                let local_target = lower_left
                    + TVector3::new(
                        (x as f32) * stride_width,
                        y as f32 * stride_height,
                        TFloat::zero(),
                    );
                let directioin = (local_target - camera_position).normalize();

                let ray_info = RayInfo {
                    x,
                    y,
                    directions: vec![directioin],
                    _marker: std::marker::PhantomData,
                };
                results.push(ray_info);
            }
        }

        results
    }
}

pub struct RayInfo<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
{
    pub x: u32,
    pub y: u32,
    pub directions: Vec<TVector3>,
    _marker: std::marker::PhantomData<TFloat>,
}
