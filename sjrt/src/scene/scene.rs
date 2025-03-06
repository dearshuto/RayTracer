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
            Primitive::Sphere(SphereData { radius: 2.0 }),
            // ボックス
            Primitive::Box(BoxData {
                width: 1.25,
                height: 1.25,
                depth: 1.25,
            }),
            // 床
            Primitive::Box(BoxData {
                width: 15.0,
                height: 0.5,
                depth: 15.0,
            }),
        ];
        let transforms = vec![
            // 照明
            Transform {
                translation: nalgebra::Vector3::new(0.0, 9.0, 0.0),
                rotation: nalgebra::Vector3::zeros(),
                scale: nalgebra::Vector3::new(3.0, 1.0, 3.0),
            },
            // ボックス
            Transform {
                translation: nalgebra::Vector3::new(0.0, 3.0, 0.0),
                rotation: nalgebra::Vector3::zeros(),
                scale: nalgebra::Vector3::new(1.0, 1.0, 1.0),
            },
            // 床
            Transform {
                translation: nalgebra::Vector3::zeros(),
                rotation: nalgebra::Vector3::zeros(),
                scale: nalgebra::Vector3::new(1.0, 1.0, 1.0),
            },
        ];
        let materials = vec![
            // 照明
            Material {
                albedo: Colors::white(),
                emission: Colors::white(),
                specular: 0.0,
            },
            // ボックス
            Material {
                albedo: Colors::white(),
                emission: Colors::black(),
                specular: 0.0,
            },
            // 床
            Material {
                albedo: Colors::white(),
                emission: Colors::black(),
                specular: 0.0,
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
                specular: 0.0,
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
                specular: 0.0,
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
                specular: 0.0,
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
                specular: 0.0,
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
                specular: 0.0,
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
                specular: 0.0,
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
                specular: 0.0,
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
                specular: 0.0,
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

    // 拡散反射と鏡面反射の比率
    // 1.0 なら完全鏡面反射
    pub specular: f32,
}
