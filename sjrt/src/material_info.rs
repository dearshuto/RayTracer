use crate::{
    traits::{IVector3, IVectorComponent3},
    Property,
};

#[derive(Debug, Copy, Clone)]
pub enum Brdf {
    Lambert,
    PerfectSpecularReflection,
}

pub struct MaterialInfo<TFloat, TVector3, TVectorComponent3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
    TVectorComponent3: IVectorComponent3<TFloat>,
{
    pub normal: TVector3,
    pub position: TVector3,
    pub property: Property<TFloat, TVectorComponent3>,
}

impl<TFloat, TVector3, TVectorComponent3> MaterialInfo<TFloat, TVector3, TVectorComponent3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
    TVectorComponent3: IVectorComponent3<TFloat>,
{
    pub fn new(
        normal: TVector3,
        position: TVector3,
        property: Property<TFloat, TVectorComponent3>,
    ) -> Self {
        Self {
            normal,
            position,
            property,
        }
    }
}
