use crate::traits::IVector3;

pub trait IBidirectionalReflectanceDistributionFunction<TFloat, TVector>
where
    TFloat: num::Float,
    TVector: IVector3<TFloat>,
{
    fn calculate(
        &self,
        normal: &TVector,
        in_direction: &TVector,
        out_direction: &TVector,
    ) -> TFloat;
}
