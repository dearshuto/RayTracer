use crate::traits::IVector3;

pub struct Ray<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
{
    pub origin: TVector3,
    pub direction: TVector3,
    pub depth: u32,
    _marker: std::marker::PhantomData<TFloat>,
}

impl<TFloat, TVector3> Ray<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
{
    pub fn new(origin: TVector3, direction: TVector3, depth: u32) -> Self {
        Self {
            origin,
            direction,
            depth,
            _marker: std::marker::PhantomData,
        }
    }
}
