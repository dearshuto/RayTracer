use crate::{
    traits::{IInnerProduct, INormalized, IVectorComponent3},
    IConstract, INorm, IOuterProduct,
};

impl<TFloat> IVectorComponent3<TFloat> for nalgebra::Vector3<TFloat>
where
    TFloat: num::Float + nalgebra::Scalar,
{
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

impl<T> IConstract<T> for nalgebra::Vector3<T>
where
    T: num::Float,
{
    fn new(x: T, y: T, z: T) -> Self {
        nalgebra::Vector3::new(x, y, z)
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

impl INorm for nalgebra::Vector3<f32> {
    fn norm(&self) -> f32 {
        self.norm()
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

impl<T: num::Float> IOuterProduct for nalgebra::Vector3<T>
where
    T: nalgebra::Scalar
        + nalgebra::ClosedAddAssign
        + nalgebra::ClosedSubAssign
        + nalgebra::ClosedMulAssign,
{
    fn cross(&self, other: &Self) -> Self {
        self.cross(other)
    }
}
