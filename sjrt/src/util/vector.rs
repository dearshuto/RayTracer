use crate::traits::IVectorComponent3;

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
