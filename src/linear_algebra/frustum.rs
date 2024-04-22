use super::{ray::Ray, vec3::Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Frustum {
    origin: Vec3,
    up: Vec3,
    right: Vec3,
    down: Vec3,
    left: Vec3,
    top_left: Vec3,
    top_right: Vec3,
    bottom_left: Vec3,
    bottom_right: Vec3,
}

impl Frustum {
    pub fn new(top_left: &Ray, top_right: &Ray, bottom_right: &Ray, bottom_left: &Ray) -> Frustum {
        let up = *Vec3::cross(top_left.get_direction(), top_right.get_direction()).normalize();
        let right =
            *Vec3::cross(top_right.get_direction(), bottom_right.get_direction()).normalize();
        let down =
            *Vec3::cross(bottom_right.get_direction(), bottom_left.get_direction()).normalize();
        let left = *Vec3::cross(bottom_left.get_direction(), top_left.get_direction()).normalize();
        Frustum {
            origin: top_left.get_origin().clone(),
            up,
            right,
            down,
            left,
            top_left: top_left.get_direction().clone(),
            top_right: top_right.get_direction().clone(),
            bottom_left: bottom_left.get_direction().clone(),
            bottom_right: bottom_right.get_direction().clone(),
        }
    }
}
