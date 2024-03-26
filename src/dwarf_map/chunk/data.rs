use bevy::prelude::*;

// vertices

pub const CEILING_VERTICES: [Vec3; 4] = [
    Vec3::new(-0.5, 0.5, -0.5),
    Vec3::new(0.5, 0.5, -0.5),
    Vec3::new(0.5, 0.5, 0.5),
    Vec3::new(-0.5, 0.5, 0.5),
];

pub const FLOOR_VERTICES: [Vec3; 4] = [
    Vec3::new(-0.5, -0.5, -0.5),
    Vec3::new(0.5, -0.5, -0.5),
    Vec3::new(0.5, -0.5, 0.5),
    Vec3::new(-0.5, -0.5, 0.5),
];

pub const RIGHT_VERTICES: [Vec3; 4] = [
    Vec3::new(0.5, -0.5, -0.5),
    Vec3::new(0.5, -0.5, 0.5),
    Vec3::new(0.5, 0.5, 0.5),
    Vec3::new(0.5, 0.5, -0.5),
];

pub const LEFT_VERTICES: [Vec3; 4] = [
    Vec3::new(-0.5, -0.5, -0.5),
    Vec3::new(-0.5, -0.5, 0.5),
    Vec3::new(-0.5, 0.5, 0.5),
    Vec3::new(-0.5, 0.5, -0.5),
];

pub const BACK_VERTICES: [Vec3; 4] = [
    Vec3::new(-0.5, -0.5, 0.5),
    Vec3::new(-0.5, 0.5, 0.5),
    Vec3::new(0.5, 0.5, 0.5),
    Vec3::new(0.5, -0.5, 0.5),
];

pub const FRONT_VERTICES: [Vec3; 4] = [
    Vec3::new(-0.5, -0.5, -0.5),
    Vec3::new(-0.5, 0.5, -0.5),
    Vec3::new(0.5, 0.5, -0.5),
    Vec3::new(0.5, -0.5, -0.5),
];

// Normals

pub const CEILING_NORMALS: [Vec3; 4] = [
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
];

pub const FLOOR_NORMALS: [Vec3; 4] = [
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
];

pub const RIGHT_NORMALS: [Vec3; 4] = [
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
];

pub const LEFT_NORMALS: [Vec3; 4] = [
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
];

pub const BACK_NORMALS: [Vec3; 4] = [
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
];

pub const FRONT_NORMALS: [Vec3; 4] = [
    Vec3::new(0.0, 0.0, -1.0),
    Vec3::new(0.0, 0.0, -1.0),
    Vec3::new(0.0, 0.0, -1.0),
    Vec3::new(0.0, 0.0, -1.0),
];

pub mod cube {

    use crate::dwarf_map::chunk::temp_mesh::TempMesh;

    use super::*;

    const INDICES: [u32; 6] = [0, 3, 1, 1, 3, 2];
    const REV_INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

    fn offset_vertices(vertices: &[Vec3; 4], offset: &Vec3) -> [Vec3; 4] {
        [
            vertices[0] + *offset,
            vertices[1] + *offset,
            vertices[2] + *offset,
            vertices[3] + *offset,
        ]
    }

    pub fn add_ceiling(mesh: &mut TempMesh, offset: &Vec3, uvs: [Vec2; 4]) {
        mesh.extend(
            &uvs,
            &CEILING_NORMALS,
            &offset_vertices(&CEILING_VERTICES, offset),
            &INDICES,
        );
    }

    #[allow(unused)]
    pub fn add_floor(mesh: &mut TempMesh, offset: &Vec3, uvs: [Vec2; 4]) {
        mesh.extend(
            &uvs,
            &FLOOR_NORMALS,
            &offset_vertices(&FLOOR_VERTICES, offset),
            &INDICES,
        );
    }

    pub fn add_bottom(mesh: &mut TempMesh, offset: &Vec3, uvs: [Vec2; 4]) {
        mesh.extend(
            &uvs,
            &FLOOR_NORMALS,
            &offset_vertices(&FLOOR_VERTICES, offset),
            &REV_INDICES,
        );
    }

    pub fn add_front(mesh: &mut TempMesh, offset: &Vec3, uvs: [Vec2; 4]) {
        mesh.extend(
            &uvs,
            &FRONT_NORMALS,
            &offset_vertices(&FRONT_VERTICES, offset),
            &REV_INDICES,
        );
    }

    pub fn add_back(mesh: &mut TempMesh, offset: &Vec3, uvs: [Vec2; 4]) {
        mesh.extend(
            &uvs,
            &BACK_NORMALS,
            &offset_vertices(&BACK_VERTICES, offset),
            &INDICES,
        );
    }

    pub fn add_right(mesh: &mut TempMesh, offset: &Vec3, uvs: [Vec2; 4]) {
        mesh.extend(
            &uvs,
            &RIGHT_NORMALS,
            &offset_vertices(&RIGHT_VERTICES, offset),
            &INDICES,
        );
    }

    pub fn add_left(mesh: &mut TempMesh, offset: &Vec3, uvs: [Vec2; 4]) {
        mesh.extend(
            &uvs,
            &LEFT_NORMALS,
            &offset_vertices(&LEFT_VERTICES, offset),
            &REV_INDICES,
        );
    }
}
