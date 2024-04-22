use super::plane::Plane;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn dot(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn subtract(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }

    pub fn add(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }

    pub fn rotate(v: &Vec3, plane: &Plane, theta: f32) -> Vec3 {
        let mut res = Vec3::subtract(v, &plane.origin);

        // rotate a around b theta rads
        // res =

        res = Vec3::add(&res, &plane.origin);
        res
    }

    pub fn normalize(&mut self) -> &mut Self {
        let size_square = self.x * self.x + self.y * self.y + self.z * self.z;
        if size_square == 0.0 {
            self.x = 1.0;
            self.y = 0.0;
            self.z = 0.0;
            log::warn!("sketch");
            return self;
        }
        let size = size_square.sqrt();
        self.x /= size;
        self.y /= size;
        self.z /= size;
        self
    }

    pub fn to_normalized(&self) -> Vec3 {
        let size_square = self.x * self.x + self.y * self.y + self.z * self.z;
        if size_square == 0.0 {
            log::warn!("sketch");
            return Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
        }
        let size = size_square.sqrt();
        Vec3 {
            x: self.x / size,
            y: self.y / size,
            z: self.z / size,
        }
    }

    pub fn len(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}
