use crate::{
    executor::ISceneStructure,
    path_tracer_ex::IHitParams,
    sampling_algorithm::IRelatedLightEnumerator,
    scene::Scene,
    traits::{EnumerateLightResult, IVectorComponent3},
    IScene, MaterialInfo, Property,
};
use rapier3d::{parry::partitioning::IndexedData, prelude::*};

pub struct RapierScene {
    sky_lower_color: nalgebra::Vector3<f32>,
    sky_upper_color: nalgebra::Vector3<f32>,
    _rigid_body_set: RigidBodySet,
    _collider_set: ColliderSet,
    _island_manager: IslandManager,
    _query_pipeline: QueryPipeline,
    _properties: Vec<Property<f32, nalgebra::Vector3<f32>>>,
    _emission_object_indices: Vec<i32>,
}

impl RapierScene {
    pub fn new_from_scene(scene: &Scene) -> Self {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut properties = Vec::new();
        let mut emission_indices = Vec::new();
        for index in 0..scene.primitives.len() {
            let transform = &scene.transforms[index];
            let rigid_body = RigidBodyBuilder::fixed()
                .translation(vector![
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z
                ])
                .build();
            let handle = rigid_body_set.insert(rigid_body);

            let collider = match &scene.primitives[index] {
                crate::scene::primitive::Primitive::Sphere(data) => {
                    ColliderBuilder::ball(data.radius).build()
                }
                crate::scene::primitive::Primitive::Box(data) => {
                    ColliderBuilder::cuboid(data.width, data.height, data.depth).build()
                }
                crate::scene::primitive::Primitive::TriMesh(data) => {
                    let vertices = data
                        .positions
                        .chunks(3)
                        .map(|x| Point::<Real>::new(x[0], x[1], x[2]))
                        .collect();
                    let indices: Vec<[u32; 3]> =
                        data.indices.chunks(3).map(|x| [x[0], x[1], x[2]]).collect();
                    ColliderBuilder::trimesh(vertices, indices).build()
                }
            };
            collider_set.insert_with_parent(collider, handle, &mut rigid_body_set);

            let material = &scene.materials[index];
            let property = Property {
                emission: material.emission.x,
                albedo: nalgebra::Vector3::new(
                    material.albedo.x,
                    material.albedo.y,
                    material.albedo.z,
                ),
                ..std::default::Default::default()
            };
            properties.push(property);

            if 0.0 < material.emission.x {
                emission_indices.push(index as i32);
            }
        }

        let island_manager = IslandManager::new();
        let mut query_pipeline = QueryPipeline::new();
        query_pipeline.update(&collider_set);
        Self {
            sky_lower_color: scene.sky.lower_color,
            sky_upper_color: scene.sky.upper_color,
            _rigid_body_set: rigid_body_set,
            _collider_set: collider_set,
            _island_manager: island_manager,
            _query_pipeline: query_pipeline,
            _properties: properties,
            _emission_object_indices: emission_indices,
        }
    }

    pub fn new() -> Self {
        let cornell_box = Scene::create_cornell_box();
        Self::new_from_scene(&cornell_box)
    }
}

impl IScene for RapierScene {
    fn cast_ray(
        &self,
        from: &nalgebra::Vector3<f32>,
        to: &nalgebra::Vector3<f32>,
    ) -> Option<MaterialInfo> {
        let line_segment = vector![to.x - from.x, to.y - from.y, to.z - from.z];
        let max_toi = line_segment.norm();
        let direction = line_segment / max_toi;
        let ray = &Ray::new(point![from.x, from.y, from.z], direction);
        let colliders = &self._collider_set;
        let solid = false;
        let filter = QueryFilter::default();
        if let Some((handle, intersection)) = self._query_pipeline.cast_ray_and_get_normal(
            &self._rigid_body_set,
            colliders,
            ray,
            max_toi,
            solid,
            filter,
        ) {
            // TODO: プロパティの検索
            let collider = colliders.get(handle).unwrap();
            let parent_handle = collider.parent().unwrap();
            let normal = nalgebra::Vector3::new(
                intersection.normal[0],
                intersection.normal[1],
                intersection.normal[2],
            );
            let position = ray.point_at(intersection.time_of_impact);
            let property = &self._properties[parent_handle.index()];
            let material = MaterialInfo::new(
                normal,
                nalgebra::Vector3::new(position[0], position[1], position[2]),
                *property,
            );
            Some(material)
        } else {
            None
        }
    }

    fn enumerate_related_lights(&self, position: &nalgebra::Vector3<f32>) -> EnumerateLightResult {
        let centers: Vec<_> = self.enumerate(position).collect();
        EnumerateLightResult { centers }
    }

    fn find_background_color(
        &self,
        _position: &nalgebra::Vector3<f32>,
        direction: &nalgebra::Vector3<f32>,
    ) -> nalgebra::Vector3<f32> {
        {
            let rate = direction
                .dot(&nalgebra::Vector3::new(0.0, 1.0, 0.0))
                .clamp(0.0, 1.0);
            if 0.0 < rate {
                rate * self.sky_upper_color + (1.0 - rate) * self.sky_lower_color
            } else {
                nalgebra::Vector3::zeros()
            }
        }
    }
}

impl<TFloat, TVector3> IRelatedLightEnumerator<TFloat, TVector3> for RapierScene
where
    TFloat: num::Float + From<f32>,
    TVector3: IVectorComponent3<TFloat>,
{
    fn enumerate(&self, _position: &TVector3) -> impl Iterator<Item = TVector3> {
        let mut results = Vec::new();
        for index in &self._emission_object_indices {
            if let Some((_, handle)) = self._collider_set.get_unknown_gen(*index as u32) {
                let collider = self._collider_set.get(handle).unwrap();
                let light_position = TVector3::new(
                    ::core::convert::From::from(collider.translation()[0]),
                    ::core::convert::From::from(collider.translation()[1]),
                    ::core::convert::From::from(collider.translation()[2]),
                );
                results.push(light_position);
            }
        }
        results.into_iter()
    }
}

impl ISceneStructure<nalgebra::Vector3<f32>, RayIntersection> for RapierScene {
    fn cast(
        &self,
        from: &nalgebra::Vector3<f32>,
        to: &nalgebra::Vector3<f32>,
    ) -> Option<RayIntersection> {
        let line_segment = vector![to.x - from.x, to.y - from.y, to.z - from.z];
        let max_toi = line_segment.norm();
        let direction = line_segment / max_toi;
        let ray = &Ray::new(point![from.x, from.y, from.z], direction);
        let colliders = &self._collider_set;
        let solid = false;
        let filter = QueryFilter::default();

        let Some((_handle, intersection)) = self._query_pipeline.cast_ray_and_get_normal(
            &self._rigid_body_set,
            colliders,
            ray,
            max_toi,
            solid,
            filter,
        ) else {
            return None;
        };

        Some(intersection)
    }
}

impl ISceneStructure<nalgebra::Vector3<f32>, HitParams> for RapierScene {
    fn cast(
        &self,
        from: &nalgebra::Vector3<f32>,
        to: &nalgebra::Vector3<f32>,
    ) -> Option<HitParams> {
        let Some(material_info) = self.cast_ray(from, to) else {
            return None;
        };

        Some(HitParams {
            normal: material_info.normal,
            position: material_info.position,
            emission: nalgebra::Vector3::new(
                material_info.property.emission,
                material_info.property.emission,
                material_info.property.emission,
            ),
            albedo: material_info.property.albedo,
        })
    }
}

pub struct HitParams {
    normal: nalgebra::Vector3<f32>,
    position: nalgebra::Vector3<f32>,
    emission: nalgebra::Vector3<f32>,
    albedo: nalgebra::Vector3<f32>,
}

impl IHitParams for HitParams {
    fn normal(&self) -> nalgebra::Vector3<f32> {
        self.normal
    }

    fn position(&self) -> nalgebra::Vector3<f32> {
        self.position
    }

    fn emission(&self) -> nalgebra::Vector3<f32> {
        self.emission
    }

    fn albedo(&self) -> nalgebra::Vector3<f32> {
        self.albedo
    }
}
