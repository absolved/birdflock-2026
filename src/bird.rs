use crate::vector2d::Vec2D;

#[derive(Debug)]
pub struct Bird {
    position: Vec2D,
    velocity: Vec2D,
}

impl Bird {
    pub fn new(position: Vec2D, velocity:Vec2D) -> Self {
        Self {position, velocity}
    }
    pub fn position(&self) -> Vec2D {
        self.position
    }
    pub fn velocity(&self) -> Vec2D {
        self.velocity
    }
    pub(crate) fn update(&mut self, acceleration: &Vec2D, dt: f64, max_speed: f64, width: f64, height: f64) {
        self.velocity = self.velocity.add(&acceleration.scale(dt)).limit(max_speed);
        self.position = self.position.add(&self.velocity.scale(dt)).wrap(width,height);
    }
}