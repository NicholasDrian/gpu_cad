use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.to_normalized(),
        }
    }
    pub fn get_origin(&self) -> &Vec3 {
        &self.origin
    }
    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }
}
