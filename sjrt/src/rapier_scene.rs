use crate::{IScene, MaterialInfo, Property, Vector3f, traits::EnumerateLightResult, scene::Scene};
use rapier3d::{parry::partitioning::IndexedData, prelude::*};

pub struct RapierScene {
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
        for index in 0..scene.primitives.len() {
            let transform = &scene.transforms[index];
            let rigid_body = RigidBodyBuilder::new_static()
                .translation(vector![transform.translation.x, transform.translation.y, transform.translation.z])
                .build();
            let handle = rigid_body_set.insert(rigid_body);

            let collider = match &scene.primitives[index] {
                crate::scene::primitive::Primitive::Sphere(data) =>
                    ColliderBuilder::ball(data.radius).build(),
                crate::scene::primitive::Primitive::Box(_) => todo!(),
            };
            collider_set.insert_with_parent(collider, handle, &mut rigid_body_set);

            let material = &scene.materials[index];
            let property =  Property {
                emission: material.emission.x,
                albedo: Vector3f::new(material.albedo.x, material.albedo.y, material.albedo.z),
                ..std::default::Default::default()
            };
            properties.push(property);
        }

        let island_manager = IslandManager::new();
        let mut query_pipeline = QueryPipeline::new();
        query_pipeline.update(&island_manager, &rigid_body_set, &collider_set);
        Self {
            _rigid_body_set: rigid_body_set,
            _collider_set: collider_set,
            _island_manager: island_manager,
            _query_pipeline: query_pipeline,
            _properties: properties,
            _emission_object_indices: vec![0, 1],
        }
    }

    pub fn new() -> Self {
        // 床
        let floor_rigid_body = RigidBodyBuilder::new_static().build();
        let floor_collier = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();

        // 光源
        let light_body = RigidBodyBuilder::new_static()
            .translation(vector![0.0, 5.0, 0.0])
            .build();
        let light_clollider = ColliderBuilder::cuboid(1.0, 0.1, 1.0).build();

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let handle = rigid_body_set.insert(floor_rigid_body);
        collider_set.insert_with_parent(floor_collier, handle, &mut rigid_body_set);
        let handle = rigid_body_set.insert(light_body);
        collider_set.insert_with_parent(light_clollider, handle, &mut rigid_body_set);

        // 右の球
        let handle = rigid_body_set.insert(RigidBodyBuilder::new_static().translation(vector![1.5, 1.0, -1.0]).build());
        collider_set.insert_with_parent(ColliderBuilder::ball(0.75).build(), handle, &mut rigid_body_set);
        // 左の球
        let handle = rigid_body_set.insert(RigidBodyBuilder::new_static().translation(vector![-2.25, 3.0, -1.5]).build());
        collider_set.insert_with_parent(ColliderBuilder::ball(1.0).build(), handle, &mut rigid_body_set);

        // 左の壁
        let handle = rigid_body_set.insert(RigidBodyBuilder::new_static().translation(vector![-5.0, 0.0, 0.0]).build());
        collider_set.insert_with_parent(ColliderBuilder::cuboid(0.5, 100.0, 100.0).build(), handle, &mut rigid_body_set);

        // 右の壁
        let handle = rigid_body_set.insert(RigidBodyBuilder::new_static().translation(vector![5.0, 0.0, 0.0]).build());
        collider_set.insert_with_parent(ColliderBuilder::cuboid(0.5, 100.0, 100.0).build(), handle, &mut rigid_body_set);

        // 奥の壁
        let handle = rigid_body_set.insert(RigidBodyBuilder::new_static().translation(vector![0.0, 0.0, -5.0]).build());
        collider_set.insert_with_parent(ColliderBuilder::cuboid(100.0, 100.0, 0.5).build(), handle, &mut rigid_body_set);

        // 天井
        let handle = rigid_body_set.insert(RigidBodyBuilder::new_static().translation(vector![0.0, 6.5, 0.0]).build());
        collider_set.insert_with_parent(ColliderBuilder::cuboid(100.0, 0.5, 100.5).build(), handle, &mut rigid_body_set);

        let island_manager = IslandManager::new();
        let mut query_pipeline = QueryPipeline::new();
        query_pipeline.update(&island_manager, &rigid_body_set, &collider_set);

        Self {
            _rigid_body_set: rigid_body_set,
            _collider_set: collider_set,
            _island_manager: island_manager,
            _query_pipeline: query_pipeline,
            _properties: vec![
                std::default::Default::default(),  // 床
                Property{ emission: 1000.0, albedo: Vector3f::new(1.0, 1.0, 1.0), ..std::default::Default::default()}, // 光源
                Property{ metaric: 0.95, ..std::default::Default::default()},  // 右の球
                Property{ metaric: 0.01, ..std::default::Default::default()},  // 左の球
                Property{ albedo: Vector3f::new(0.6, 0.0, 0.0), ..std::default::Default::default()},  // 左の壁
                Property{ albedo: Vector3f::new(0.0, 0.5, 0.0), ..std::default::Default::default()},  // 右の壁
                std::default::Default::default(),  // 奥の壁
                Property{ albedo: Vector3f::new(0.0, 0.0, 0.7), ..std::default::Default::default()},  // 天井
            ],
            _emission_object_indices: vec![1],
        }
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
                let light_position = Vector3f::new(collider.translation()[0], collider.translation()[1], collider.translation()[2]);
                results.push(light_position);
            }
        }
        EnumerateLightResult{ centers: results }
    }
}
