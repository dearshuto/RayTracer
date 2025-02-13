pub trait IBidirectionalReflectanceDistributionFunction<TFloat, TVector>
where
    TFloat: num::Float,
{
    fn calculate(
        &self,
        normal: &TVector,
        in_direction: &TVector,
        out_direction: &TVector,
    ) -> TFloat;
}
