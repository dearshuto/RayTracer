use crate::traits::{IVector3, IVectorComponent3};

impl<TFloat> IVectorComponent3<TFloat> for nalgebra::Vector3<TFloat>
where
    TFloat: num::Float + nalgebra::Scalar,
{
    fn new(x: TFloat, y: TFloat, z: TFloat) -> Self {
        Self::new(x, y, z)
    }

    fn x(&self) -> TFloat {
        self.x
    }

    fn y(&self) -> TFloat {
        self.y
    }

    fn z(&self) -> TFloat {
        self.z
    }

    fn set_x(&mut self, x: TFloat) {
        self.x = x;
    }

    fn set_y(&mut self, y: TFloat) {
        self.y = y;
    }

    fn set_z(&mut self, z: TFloat) {
        self.z = z;
    }
}

impl<TFloat> IVector3<TFloat> for nalgebra::Vector3<TFloat>
where
    TFloat: num::Float + nalgebra::Scalar + nalgebra::SimdComplexField + nalgebra::SimdRealField,
{
    fn zero() -> Self {
        Self::zeros()
    }

    fn dot(&self, other: &Self) -> TFloat {
        self.dot(other)
    }

    fn normalize(&self) -> Self {
        self.normalize()
    }

    fn length(&self) -> TFloat {
        self.norm()
    }
}
