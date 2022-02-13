use crate::Vector3f;

use super::primitive::{Primitive, TriMeshData};

pub struct Colors;
impl Colors {
    pub fn white() -> Vector3f {
        Vector3f::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Vector3f {
        Vector3f::new(0.0, 0.0, 0.0)
    }

    pub fn red() -> Vector3f {
        Vector3f::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Vector3f {
        Vector3f::new(0.0, 1.0, 0.0)
    }
}

pub struct Scene {
    pub sky: Sky,
    pub primitives: Vec<Primitive>,
    pub transforms: Vec<Transform>,
    pub materials: Vec<Material>,
}

impl Scene {
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
                emission: Vector3f::new(30000.0, 30000.0, 30000.0),
            });
        }

        // Ceiling
        {
            let tri_mesh_data = TriMeshData {
                positions: vec![
                    0.0, 5.488, 0.0,
                    0.0, 5.488, 2.27,
                    5.56, 5.488, 5.5592,
                    5.56, 5.488, 0.0,

                    0.0, 5.488, 2.27,
                    0.0, 5.488, 3.32,
                    2.13, 5.488, 3.32,
                    2.13, 5.488, 2.27,

                    3.43, 5.488, 2.27,
                    3.43, 5.488, 3.32,
                    5.56, 5.488, 3.32,
                    5.56, 5.488, 2.27,

                    0.0, 5.488, 3.32,
                    0.0, 5.488, 5.592,
                    5.536, 5.488, 5.592,
                    5.56, 5.488, 3.32
                ],
                indices: vec![
                    0, 1, 2, 0, 2, 3,
                    4, 5, 6, 4, 6, 7,
                    8, 9, 10, 8, 10, 11,
                    12, 13, 14, 12, 14, 15,
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
                emission: Vector3f::zero(),
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
                emission: Vector3f::zero(),
            });
        }

        Self {
            sky: Sky {
                lower_color: Vector3f::zero(),
                upper_color: Vector3f::zero(),
            },
            primitives,
            transforms,
            materials,
        }
    }
}

pub struct Sky {
    pub lower_color: Vector3f,
    pub upper_color: Vector3f,
}

pub struct Transform {
    pub translation: Vector3f,
    pub rotation: Vector3f,
    pub scale: Vector3f,
}

impl Transform {
    pub fn new_with_translation(translation: &Vector3f) -> Self {
        Self {
            translation: *translation,
            rotation: Vector3f::zero(),
            scale: Vector3f::zero(),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vector3f::zero(),
            rotation: Vector3f::zero(),
            scale: Vector3f::new(1.0, 1.0, 1.0),
        }
    }
}

pub struct Material {
    pub albedo: Vector3f,
    pub emission: Vector3f,
}
