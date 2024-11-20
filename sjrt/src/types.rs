use crate::traits::IVectorComponent3;

pub struct Colors<TFloat, TVectorComponent>
where
    TFloat: num::Float,
    TVectorComponent: IVectorComponent3<TFloat>,
{
    _marker: std::marker::PhantomData<(TFloat, TVectorComponent)>,
}

impl<TVectorComponent> Colors<f32, TVectorComponent>
where
    TVectorComponent: IVectorComponent3<f32>,
{
    pub fn white() -> TVectorComponent {
        TVectorComponent::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> TVectorComponent {
        TVectorComponent::new(0.0, 0.0, 0.0)
    }

    pub fn red() -> TVectorComponent {
        TVectorComponent::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> TVectorComponent {
        TVectorComponent::new(0.0, 1.0, 0.0)
    }
}
