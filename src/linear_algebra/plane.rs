use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    pub normal: Vec3,
    pub origin: Vec3,
}
