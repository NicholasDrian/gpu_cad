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

    pub fn normalize(&mut self) {
        let size_square = self.x * self.x + self.y * self.y + self.z * self.z;
        if size_square == 0.0 {
            return;
        }
        let size = size_square.sqrt();
        self.x /= size;
        self.y /= size;
        self.z /= size;
    }

    pub fn len(self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}
