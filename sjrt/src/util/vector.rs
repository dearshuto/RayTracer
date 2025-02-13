use crate::traits::{IInnerProduct, INormalized, IVectorComponent3};

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

impl<T> INormalized for nalgebra::Vector3<T>
where
    T: nalgebra::Scalar + nalgebra::SimdComplexField,
{
    fn normalized(&self) -> Self {
        self.normalize()
    }
}

impl<T> IInnerProduct<T> for nalgebra::Vector3<T>
where
    T: num::Float
        + num::Zero
        + nalgebra::Scalar
        + nalgebra::ClosedAddAssign
        + nalgebra::ClosedMulAssign,
{
    fn dot(&self, other: &Self) -> T {
        self.dot(other)
    }
}
