use crate::Colors;

use super::primitive::{BoxData, Primitive, SphereData, TriMeshData};

pub struct Scene {
    pub sky: Sky,
    pub primitives: Vec<Primitive<f32>>,
    pub transforms: Vec<Transform>,
    pub materials: Vec<Material>,
}

impl Scene {
    pub fn box_point_light() -> Self {
        let sky = Sky {
            lower_color: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            upper_color: nalgebra::Vector3::new(0.1, 0.2, 0.3),
        };
        let primitives = vec![
            // 照明
            Primitive::Sphere(SphereData { radius: 0.5 }),
        ];
        let transforms = vec![
            // 照明
            Transform {
                translation: nalgebra::Vector3::new(0.0, 0.0, 0.0),
                rotation: nalgebra::Vector3::zeros(),
                scale: nalgebra::Vector3::new(1.0, 1.0, 1.0),
            },
        ];
        let materials = vec![
            // 照明
            Material {
                albedo: Colors::white(),
                emission: Colors::white(),
            },
        ];

        Self {
            sky,
            primitives,
            transforms,
            materials,
        }
    }

    pub fn create_cornell_box() -> Self {
        let mut primitives = Vec::new();
        let mut transforms = Vec::new();
        let mut materials = Vec::new();

        // Floor
        {
            let tri_mesh_data = TriMeshData {
                positions: vec![
                    5.528, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.592, 5.496, 0.0, 5.592,
                ],
                indices: vec![0, 1, 2, 0, 3, 2],
            };
            primitives.push(Primitive::TriMesh(tri_mesh_data));
            transforms.push(std::default::Default::default());
            materials.push(Material {
                albedo: Colors::white(),
                emission: Colors::black(),
            });
        }

        // Light
        {
            let tri_mesh_data = TriMeshData {
                positions: vec![
                    3.430, 5.488, 2.270, 3.430, 5.488, 3.320, 2.130, 5.488, 3.320, 2.130, 5.488,
                    2.270,
                ],
                indices: vec![0, 1, 2, 0, 2, 3],
            };
            primitives.push(Primitive::TriMesh(tri_mesh_data));
            transforms.push(std::default::Default::default());
            materials.push(Material {
                albedo: Colors::white(),
                emission: nalgebra::Vector3::new(30000.0, 30000.0, 30000.0),
            });
        }

        // Ceiling
        {
            let tri_mesh_data = TriMeshData {
                positions: vec![
                    0.0, 5.488, 0.0, 0.0, 5.488, 2.27, 5.56, 5.488, 5.5592, 5.56, 5.488, 0.0, 0.0,
                    5.488, 2.27, 0.0, 5.488, 3.32, 2.13, 5.488, 3.32, 2.13, 5.488, 2.27, 3.43,
                    5.488, 2.27, 3.43, 5.488, 3.32, 5.56, 5.488, 3.32, 5.56, 5.488, 2.27, 0.0,
                    5.488, 3.32, 0.0, 5.488, 5.592, 5.536, 5.488, 5.592, 5.56, 5.488, 3.32,
                ],
                indices: vec![
                    0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15,
                ],
            };
            primitives.push(Primitive::TriMesh(tri_mesh_data));
            transforms.push(std::default::Default::default());
            materials.push(Material {
                albedo: Colors::white(),
                emission: Colors::black(),
            });
        }

        // Back Wall
        {
            let tri_mesh_data = TriMeshData {
                positions: vec![
                    5.560, 0.0, 5.592, 0.0, 0.0, 5.592, 0.0, 5.488, 5.592, 5.560, 5.488, 5.592,
                ],
                indices: vec![0, 1, 2, 0, 2, 3],
            };
            primitives.push(Primitive::TriMesh(tri_mesh_data));
            transforms.push(std::default::Default::default());
            materials.push(Material {
                albedo: Colors::white(),
                emission: Colors::black(),
            });
        }

        // Right Wall
        {
            let tri_mesh_data = TriMeshData {
                positions: vec![
                    0.0, 0.0, 5.592, 0.0, 0.0, 0.0, 0.0, 5.488, 0.0, 0.0, 5.488, 5.592,
                ],
                indices: vec![0, 1, 2, 0, 2, 3],
            };
            primitives.push(Primitive::TriMesh(tri_mesh_data));
            transforms.push(std::default::Default::default());
            materials.push(Material {
                albedo: Colors::green(),
                emission: Colors::black(),
            });
        }

        // Left Wall
        {
            let tri_mesh_data = TriMeshData {
                positions: vec![
                    5.528, 0.0, 0.0, 5.496, 0.0, 5.592, 5.560, 5.488, 5.592, 5.560, 5.488, 0.0,
                ],
                indices: vec![0, 1, 2, 0, 3, 2],
            };
            primitives.push(Primitive::TriMesh(tri_mesh_data));
            transforms.push(std::default::Default::default());
            materials.push(Material {
                albedo: Colors::red(),
                emission: Colors::black(),
            });
        }

        // Short block
        {
            let tri_mesh_data = TriMeshData {
                positions: vec![
                    1.300, 1.650, 0.650, 0.820, 1.650, 2.250, 2.400, 1.650, 2.720, 2.900, 1.650,
                    1.140, 2.900, 0.0, 1.140, 2.900, 1.650, 1.140, 2.400, 1.650, 2.720, 2.400, 0.0,
                    2.720, 1.300, 0.0, 0.650, 1.300, 1.650, 0.650, 2.900, 1.650, 1.140, 2.900, 0.0,
                    1.140, 0.820, 0.0, 2.250, 0.820, 1.650, 2.250, 1.300, 1.650, 0.650, 1.300, 0.0,
                    0.650, 2.400, 0.0, 2.720, 2.400, 1.650, 2.720, 0.820, 1.650, 2.250, 0.820, 0.0,
                    2.250,
                ],
                indices: vec![
                    0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14,
                    15, 16, 17, 18, 16, 18, 19,
                ],
            };
            primitives.push(Primitive::TriMesh(tri_mesh_data));
            transforms.push(std::default::Default::default());
            materials.push(Material {
                albedo: Colors::white(),
                emission: nalgebra::Vector3::zeros(),
            });
        }

        // Tall block
        {
            let tri_mesh_data = TriMeshData {
                positions: vec![
                    4.230, 3.300, 2.470, 2.650, 3.300, 2.960, 3.140, 3.300, 4.560, 4.720, 3.300,
                    4.060, 4.230, 0.0, 2.470, 4.230, 3.300, 2.470, 4.720, 3.300, 4.060, 4.720, 0.0,
                    4.060, 4.720, 0.0, 4.060, 4.720, 3.300, 4.060, 3.140, 3.300, 4.560, 3.140, 0.0,
                    4.560, 3.140, 0.0, 4.560, 3.140, 3.300, 4.560, 2.650, 3.300, 2.960, 2.650, 0.0,
                    2.960, 2.650, 0.0, 2.960, 2.650, 3.300, 2.960, 4.230, 3.300, 2.470, 4.230, 0.0,
                    2.470,
                ],
                indices: vec![
                    0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14,
                    15, 16, 17, 18, 16, 18, 19,
                ],
            };
            primitives.push(Primitive::TriMesh(tri_mesh_data));
            transforms.push(std::default::Default::default());
            materials.push(Material {
                albedo: Colors::white(),
                emission: nalgebra::Vector3::zeros(),
            });
        }

        Self {
            sky: Sky {
                lower_color: nalgebra::Vector3::zeros(),
                upper_color: nalgebra::Vector3::zeros(),
            },
            primitives,
            transforms,
            materials,
        }
    }
}

pub struct Sky {
    pub lower_color: nalgebra::Vector3<f32>,
    pub upper_color: nalgebra::Vector3<f32>,
}

pub struct Transform {
    pub translation: nalgebra::Vector3<f32>,
    pub rotation: nalgebra::Vector3<f32>,
    pub scale: nalgebra::Vector3<f32>,
}

impl Transform {
    pub fn new_with_translation(translation: &nalgebra::Vector3<f32>) -> Self {
        Self {
            translation: *translation,
            rotation: nalgebra::Vector3::zeros(),
            scale: nalgebra::Vector3::zeros(),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: nalgebra::Vector3::zeros(),
            rotation: nalgebra::Vector3::zeros(),
            scale: nalgebra::Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

pub struct Material {
    pub albedo: nalgebra::Vector3<f32>,
    pub emission: nalgebra::Vector3<f32>,
}
