use crate::{traits::IVectorComponent3, Brdf};

#[derive(Debug, Copy, Clone)]
pub struct Property<TFloat, TVectorComponent>
where
    TFloat: num::Float,
    TVectorComponent: IVectorComponent3<TFloat>,
{
    pub metaric: TFloat,
    pub roughness: TFloat,
    pub emission: TFloat,
    pub diffuse_brdf: Brdf,
    pub specular_brdf: Brdf,
    pub albedo: TVectorComponent,
}

impl<TFloat, TVectorComponent> Default for Property<TFloat, TVectorComponent>
where
    TFloat: num::Float,
    TVectorComponent: IVectorComponent3<TFloat>,
{
    fn default() -> Self {
        Self {
            metaric: TFloat::zero(),
            roughness: TFloat::zero(),
            emission: TFloat::zero(),
            diffuse_brdf: Brdf::Lambert,
            specular_brdf: Brdf::PerfectSpecularReflection,
            albedo: TVectorComponent::new(
                TFloat::from(1.0).unwrap(),
                TFloat::from(1.0).unwrap(),
                TFloat::from(1.0).unwrap(),
            ),
        }
    }
}
