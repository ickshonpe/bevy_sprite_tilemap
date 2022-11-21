use std::f32::consts::PI;

use bevy::math::vec3;
use bevy::prelude::*;

fn print_muls(transform: GlobalTransform, point: Vec3) {
    let a = transform.transform_point(point);
    let b = transform.affine().transform_point3(point);
    let c = transform.affine().transform_vector3(point);
    let d = transform.affine().inverse().transform_vector3(point);
    println!("mul_vec3 => {a}");
    println!("transform_point3 => {b}");
    println!("transform_vector3 => {c}");
    println!("inverse -> transform_vector3 => {d}");
}

fn main() {
    let world_point = vec3(0., 1.0, 0.0);
    let transform = Transform {
        rotation: Quat::from_rotation_z(PI / 4.),
        scale: Vec3::new(1.0, 2.0, 1.0),
        ..Default::default()
    }
    .into();

    print_muls(transform, world_point);
}
