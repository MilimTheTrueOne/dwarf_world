use bevy::prelude::*;

const THIRD: f32 = 1.0 / 3.0;
const THIRD2: f32 = 2.0 / 3.0;

pub const CEILING_UV: [Vec2; 4] = [
    Vec2::new(0.0, 0.0),
    Vec2::new(0.0, THIRD),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, THIRD),
];
pub const WALL_UV: [Vec2; 4] = [
    Vec2::new(0.0, THIRD),
    Vec2::new(0.0, THIRD2),
    Vec2::new(1.0, THIRD),
    Vec2::new(1.0, THIRD2),
];
pub const FLOOR_UV: [Vec2; 4] = [
    Vec2::new(0.0, THIRD2),
    Vec2::new(0.0, 1.0),
    Vec2::new(1.0, THIRD2),
    Vec2::new(1.0, 1.0),
];

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

    pub fn add_ceiling(mesh: &mut TempMesh, offset: &Vec3) {
        mesh.extend(
            &CEILING_UV,
            &CEILING_NORMALS,
            &offset_vertices(&CEILING_VERTICES, offset),
            &INDICES,
        );
    }

    #[allow(unused)]
    pub fn add_floor(mesh: &mut TempMesh, offset: &Vec3) {
        mesh.extend(
            &FLOOR_UV,
            &FLOOR_NORMALS,
            &offset_vertices(&FLOOR_VERTICES, offset),
            &INDICES,
        );
    }

    pub fn add_bottom(mesh: &mut TempMesh, offset: &Vec3) {
        mesh.extend(
            &FLOOR_UV,
            &FLOOR_NORMALS,
            &offset_vertices(&FLOOR_VERTICES, offset),
            &REV_INDICES,
        );
    }

    pub fn add_front(mesh: &mut TempMesh, offset: &Vec3) {
        mesh.extend(
            &WALL_UV,
            &FRONT_NORMALS,
            &offset_vertices(&FRONT_VERTICES, offset),
            &REV_INDICES,
        );
    }

    pub fn add_back(mesh: &mut TempMesh, offset: &Vec3) {
        mesh.extend(
            &WALL_UV,
            &BACK_NORMALS,
            &offset_vertices(&BACK_VERTICES, offset),
            &INDICES,
        );
    }

    pub fn add_right(mesh: &mut TempMesh, offset: &Vec3) {
        mesh.extend(
            &WALL_UV,
            &RIGHT_NORMALS,
            &offset_vertices(&RIGHT_VERTICES, offset),
            &INDICES,
        );
    }

    pub fn add_left(mesh: &mut TempMesh, offset: &Vec3) {
        mesh.extend(
            &WALL_UV,
            &LEFT_NORMALS,
            &offset_vertices(&LEFT_VERTICES, offset),
            &REV_INDICES,
        );
    }
}
