use crate::traits::{IVector3, IVectorComponent3};

pub struct Vector3<TFloat>
where
    TFloat: num::Float + nalgebra::Scalar,
{
    vector: nalgebra::Vector3<TFloat>,
}

impl<TFloat> IVectorComponent3<TFloat> for Vector3<TFloat>
where
    TFloat: num::Float + nalgebra::Scalar,
{
    fn new(x: TFloat, y: TFloat, z: TFloat) -> Self {
        Self {
            vector: nalgebra::Vector3::new(x, y, z),
        }
    }

    fn x(&self) -> TFloat {
        self.vector[0]
    }

    fn y(&self) -> TFloat {
        self.vector[1]
    }

    fn z(&self) -> TFloat {
        self.vector[2]
    }

    fn set_x(&mut self, x: TFloat) {
        self.vector[0] = x;
    }

    fn set_y(&mut self, y: TFloat) {
        self.vector[1] = y;
    }

    fn set_z(&mut self, z: TFloat) {
        self.vector[2] = z;
    }
}

impl<TFloat> IVector3<TFloat> for Vector3<TFloat>
where
    TFloat: num::Float + nalgebra::Scalar + nalgebra::SimdComplexField + nalgebra::SimdRealField,
{
    fn zero() -> Self {
        Self {
            vector: nalgebra::Vector3::<TFloat>::zeros(),
        }
    }

    fn dot(&self, other: &Self) -> TFloat {
        self.vector.dot(&other.vector)
    }

    fn normalize(&self) -> Self {
        Self {
            vector: self.vector.normalize(),
        }
    }

    fn length(&self) -> TFloat {
        self.vector.norm()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_f32() {
        let _vector = Vector3::new(0.0f32, 0.0, 0.0);
    }

    #[test]
    fn new_f64() {
        let _vector = Vector3::new(0.0, 0.0, 0.0);
    }
}
