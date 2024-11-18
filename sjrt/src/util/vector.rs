use crate::traits::IVector3;

pub struct Vector3<TFloat>
where
    TFloat: num::Float + nalgebra::Scalar,
{
    vector: nalgebra::Vector3<TFloat>,
}

impl<TFloat> IVector3<TFloat> for Vector3<TFloat>
where
    TFloat: num::Float
        + nalgebra::Scalar
        + nalgebra::ClosedAdd
        + nalgebra::ClosedMul
        + nalgebra::SimdComplexField
        + nalgebra::SimdRealField,
{
    fn new(x: TFloat, y: TFloat, z: TFloat) -> Self {
        Self {
            vector: nalgebra::Vector3::new(x, y, z),
        }
    }

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
