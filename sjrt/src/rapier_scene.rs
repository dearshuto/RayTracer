use crate::{scene::Scene, traits::EnumerateLightResult, IScene, MaterialInfo, Property, Vector3f};
use rapier3d::{parry::partitioning::IndexedData, prelude::*};

pub struct RapierScene {
    sky_lower_color: Vector3f,
    sky_upper_color: Vector3f,
    _rigid_body_set: RigidBodySet,
    _collider_set: ColliderSet,
    _island_manager: IslandManager,
    _query_pipeline: QueryPipeline,
    _properties: Vec<Property>,
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
            let rigid_body = RigidBodyBuilder::new_static()
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
                crate::scene::primitive::Primitive::Box(_) => todo!(),
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
                albedo: Vector3f::new(material.albedo.x, material.albedo.y, material.albedo.z),
                ..std::default::Default::default()
            };
            properties.push(property);

            if 0.0 < material.emission.x {
                emission_indices.push(index as i32);
            }
        }

        let island_manager = IslandManager::new();
        let mut query_pipeline = QueryPipeline::new();
        query_pipeline.update(&island_manager, &rigid_body_set, &collider_set);
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
    fn cast_ray(&self, from: &Vector3f, to: &Vector3f) -> Option<MaterialInfo> {
        let line_segment = vector![to.x - from.x, to.y - from.y, to.z - from.z];
        let max_toi = line_segment.norm();
        let direction = line_segment / max_toi;
        let ray = &Ray::new(point![from.x, from.y, from.z], direction);
        let colliders = &self._collider_set;
        let solid = false;
        let query_groups = InteractionGroups::all();
        let filter = None;
        if let Some((handle, intersection)) = self._query_pipeline.cast_ray_and_get_normal(
            colliders,
            ray,
            max_toi,
            solid,
            query_groups,
            filter,
        ) {
            // TODO: プロパティの検索
            let collider = colliders.get(handle).unwrap();
            let parent_handle = collider.parent().unwrap();
            let normal = Vector3f::new(
                intersection.normal[0],
                intersection.normal[1],
                intersection.normal[2],
            );
            let position = ray.point_at(intersection.toi);
            let property = &self._properties[parent_handle.index()];
            let material = MaterialInfo::new(
                normal,
                Vector3f::new(position[0], position[1], position[2]),
                *property,
            );
            Some(material)
        } else {
            None
        }
    }

    fn enumerate_related_lights(&self, _position: &Vector3f) -> EnumerateLightResult {
        let mut results = Vec::new();
        for index in &self._emission_object_indices {
            if let Some((_, handle)) = self._collider_set.get_unknown_gen(*index as u32) {
                let collider = self._collider_set.get(handle).unwrap();
                let light_position = Vector3f::new(
                    collider.translation()[0],
                    collider.translation()[1],
                    collider.translation()[2],
                );
                results.push(light_position);
            }
        }
        EnumerateLightResult { centers: results }
    }

    fn find_background_color(&self, _position: &Vector3f, direction: &Vector3f) -> Vector3f {
        {
            let rate = direction.dot(&Vector3f::new(0.0, 1.0, 0.0)).clamp(0.0, 1.0);
            if 0.0 < rate {
                let sky_color = rate * self.sky_upper_color + (1.0 - rate) * self.sky_lower_color;
                sky_color
            } else {
                Vector3f::zero()
            }
        }
    }
}
