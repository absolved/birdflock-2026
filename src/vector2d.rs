#[derive(Debug, Clone, Copy)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    pub fn magnitude(&self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()    
    }

    pub fn subtract(&self, other: &Vec2D) -> Vec2D {
        Vec2D {
            x:self.x-other.x,
            y:self.y-other.y
        }
    }
    pub fn add(&self, other: &Vec2D) -> Vec2D {
        Vec2D {
            x:self.x+other.x,
            y:self.y+other.y
        }
    }
    pub fn scale(&self, scalar:f64) -> Vec2D {
        Vec2D {
            x:scalar*self.x,
            y:scalar*self.y
        }
    }
    pub fn limit(&self, max_magnitude: f64) -> Vec2D {
        let v_mag = self.magnitude();
        if v_mag > max_magnitude {
            self.scale(max_magnitude/v_mag)
        } else {
            *self
        }
    }
    pub fn wrap(&self, width: f64, height: f64) -> Vec2D {
        Vec2D {
            x: self.x.rem_euclid(width),
            y: self.y.rem_euclid(height)
        }
    }    
    pub fn periodic_displacement(&self, other: &Vec2D, width: f64, height: f64,) -> Vec2D {
        let displacement = other.subtract(self);
        let half_domain = Vec2D {
            x: width/2.0, y: height/2.0
        };
        Vec2D {
            x: (displacement.x+half_domain.x).rem_euclid(width) - half_domain.x,
            y: (displacement.y+half_domain.y).rem_euclid(height) - half_domain.y
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 1e-12;

    #[test]
    fn test_magnitude() {
        let v = Vec2D {x:3.0, y: 4.0};

        assert!((v.magnitude()-5.0).abs() < EPSILON);
    }

    #[test]
    fn test_limit() {
        let v = Vec2D { x: 3.0, y: 4.0};
        let limited = v.limit(2.0);

        assert!((limited.x - 1.2).abs() < EPSILON);
        assert!((limited.y - 1.6).abs() < EPSILON);
    }

    #[test]
    fn test_add() {
        let v = Vec2D { x: 2.0, y: 3.0};
        let w = Vec2D { x: 4.0, y: -1.0};
        let sum = v.add(&w);

        assert!((sum.x - 6.0).abs() < EPSILON, "Expected x = 6.0 got {}", sum.x);
        assert!((sum.y - 2.0).abs() < EPSILON, "Expected y = 2.0 got {}", sum.y);
    }

    #[test]
    fn test_subtract() {
        let v = Vec2D { x: 5.0, y: 7.0};
        let w = Vec2D { x: 3.0, y: -4.0};
        let diff = v.subtract(&w);

        assert!((diff.x - 2.0).abs() < EPSILON, "Expected x = 2.0 got {}", diff.x);
        assert!((diff.y - 11.0).abs() < EPSILON, "Expected y = 11.0 got {}", diff.y);
    }

    #[test]
    fn test_scale() {
        let w = Vec2D { x: 3.0, y: -4.0};
        let w_scaled = w.scale(2.5);

        assert!((w_scaled.x-7.5).abs() < EPSILON, "Expected x = 7.5 got {}", w_scaled.x);
        assert!((w_scaled.y+10.0).abs() < EPSILON, "Expected y = -10.0 got {}", w_scaled.y);
    }

    #[test]
    fn test_wrap() {
        let w = Vec2D { x: 105.0, y: -5.0};
        let width = 100.0;
        let height = 100.0;
        let w_wrapped = w.wrap(width, height);

        assert!((w_wrapped.x-5.0).abs() < EPSILON, "Expected x = 5.0 got {}", w_wrapped.x);
        assert!((w_wrapped.y-95.0).abs() < EPSILON, "Expected y = 95.0 got {}", w_wrapped.y);
    }

    #[test]
    fn test_periodic_displacement() {
        let v = Vec2D { x: 98.0, y: 50.0};
        let w = Vec2D { x: 2.0, y: 50.0};
        let width = 100.0;
        let height = 100.0;
        let v_periodic_diff = v.periodic_displacement(&w, width, height);

        assert!((v_periodic_diff.x-4.0).abs() < EPSILON, "Expected x = 4.0 got {}", v_periodic_diff.x);
        assert!((v_periodic_diff.y-0.0).abs() < EPSILON, "Expected y = 0.0 got {}", v_periodic_diff.y);
    }
}