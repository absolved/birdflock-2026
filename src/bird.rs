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
    pub(crate) fn update(&mut self, acceleration: &Vec2D, dt: f64, max_speed: f64, sim_width: f64, sim_height: f64) {
        self.velocity = self.velocity.add(&acceleration.scale(dt)).limit(max_speed);
        self.position = self.position.add(&self.velocity.scale(dt)).wrap(sim_width,sim_height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 1e-12;

    #[test]
    fn test_new() {
        let position = Vec2D {x: 10.0, y: 20.0};
        let velocity = Vec2D {x: 3.0, y: -4.0};
        let bird = Bird::new(position, velocity);
        let p = bird.position();
        let v = bird.velocity();

        assert!((p.x-10.0).abs() < EPSILON, "Expected bird.position.x = 10.0 got {}", p.x);
        assert!((p.y-20.0).abs() < EPSILON, "Expected bird.position.y = 20.0 got {}", p.y);
        assert!((v.x-3.0).abs() < EPSILON, "Expected bird.velocity.x = 3.0 got {}", v.x);
        assert!((v.y+4.0).abs() < EPSILON, "Expected bird.velocity.y = -4.0 got {}",v.y);
    }

    #[test]
    fn test_update_constant_velocity() {
        let position = Vec2D {x: 10.0, y: 20.0};
        let velocity = Vec2D {x: 3.0, y: -4.0};
        let acceleration = Vec2D {x: 0.0, y: 0.0};
        let mut bird = Bird::new(position, velocity);
        let dt = 0.5;
        let max_speed = 10.0;
        let sim_width = 100.0;
        let sim_height = 100.0;

        bird.update(&acceleration, dt, max_speed, sim_width, sim_height);
        let p = bird.position();
        let v = bird.velocity();
        assert!((p.x-11.5).abs() < EPSILON, "Expected bird.position.x = 11.5 got {}", p.x);
        assert!((p.y-18.0).abs() < EPSILON, "Expected bird.position.y = 18.0 got {}", p.y);
        assert!((v.x-3.0).abs() < EPSILON, "Expected bird.velocity.x = 3.0 got {}", v.x);
        assert!((v.y+4.0).abs() < EPSILON, "Expected bird.velocity.y = -4.0 got {}",v.y);
    }

    #[test]
    fn test_update_with_acceleration() {
        let position = Vec2D {x: 10.0, y: 20.0};
        let velocity = Vec2D {x: 3.0, y: -4.0};
        let acceleration = Vec2D {x: 2.0, y: 6.0};
        let mut bird = Bird::new(position, velocity);
        let dt = 0.5;
        let max_speed = 10.0;
        let sim_width = 100.0;
        let sim_height = 100.0;

        bird.update(&acceleration, dt, max_speed, sim_width, sim_height);
        let p = bird.position();
        let v = bird.velocity();
        assert!((p.x-12.0).abs() < EPSILON, "Expected bird.position.x = 12.0 got {}", p.x);
        assert!((p.y-19.5).abs() < EPSILON, "Expected bird.position.y = 19.5 got {}", p.y);
        assert!((v.x-4.0).abs() < EPSILON, "Expected bird.velocity.x = 4.0 got {}", v.x);
        assert!((v.y+1.0).abs() < EPSILON, "Expected bird.velocity.y = -1.0 got {}",v.y);
    }

    #[test]
    fn test_update_speed_limit() {
        let position = Vec2D {x: 10.0, y: 20.0};
        let velocity = Vec2D {x: 3.0, y: 4.0};
        let acceleration = Vec2D {x: 6.0, y: 8.0};
        let mut bird = Bird::new(position, velocity);
        let dt = 1.0;
        let max_speed = 5.0;
        let sim_width = 100.0;
        let sim_height = 100.0;

        bird.update(&acceleration, dt, max_speed, sim_width, sim_height);
        let p = bird.position();
        let v = bird.velocity();
        assert!((p.x-13.0).abs() < EPSILON, "Expected bird.position.x = 13.0 got {}", p.x);
        assert!((p.y-24.0).abs() < EPSILON, "Expected bird.position.y = 24.0 got {}", p.y);
        assert!((v.x-3.0).abs() < EPSILON, "Expected bird.velocity.x = 3.0 got {}", v.x);
        assert!((v.y-4.0).abs() < EPSILON, "Expected bird.velocity.y = 4.0 got {}",v.y);
        assert!((v.magnitude()-max_speed).abs() < EPSILON, "Expected speed = {} got {}",max_speed,v.magnitude());
    }

    #[test]
    fn test_update_wrap() {
        let position = Vec2D {x: 98.0, y: 2.0};
        let velocity = Vec2D {x: 5.0, y: -6.0};
        let acceleration = Vec2D {x: 0.0, y: 0.0};
        let mut bird = Bird::new(position, velocity);
        let dt = 1.0;
        let max_speed = 10.0;
        let sim_width = 100.0;
        let sim_height = 100.0;

        bird.update(&acceleration, dt, max_speed, sim_width, sim_height);
        let p = bird.position();
        let v = bird.velocity();
        assert!((p.x-3.0).abs() < EPSILON, "Expected bird.position.x = 3.0 got {}", p.x);
        assert!((p.y-96.0).abs() < EPSILON, "Expected bird.position.y = 96.0 got {}", p.y);
        assert!((v.x-5.0).abs() < EPSILON, "Expected bird.velocity.x = 5.0 got {}", v.x);
        assert!((v.y+6.0).abs() < EPSILON, "Expected bird.velocity.y = -6.0 got {}",v.y);
    }
}