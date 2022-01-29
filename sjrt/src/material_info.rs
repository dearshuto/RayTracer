use crate::{Vector3f, Property};

pub struct MaterialInfo
{
    pub normal: Vector3f,
    pub position: Vector3f,
    pub property: Property,
}

impl MaterialInfo
{
    pub fn new(normal: Vector3f, position: Vector3f, property: Property) -> Self
    {
        Self{
            normal,
            position,
            property
        }
    }
}
