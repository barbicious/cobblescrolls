use nalgebra_glm::Vec3;

pub struct Ray {
    start: Vec3,
    end: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(position: Vec3, direction: Vec3) -> Self {
        Self {
            start: position,
            end: position,
            direction,
        }
    }

    pub fn step(&mut self, scale: f32) {
        let yaw = (self.direction.x + 90.0).to_radians();
        let pitch = -self.direction.y.to_radians();

        self.end.x -= yaw.cos() * scale;
        self.end.y -= pitch.tan() * scale;
        self.end.z -= yaw.sin() * scale;
    }

    pub fn distance(&self) -> f32 {
        nalgebra_glm::distance(&self.start, &self.end)
    }

    pub fn end(&self) -> &Vec3 {
        &self.end
    }
}
