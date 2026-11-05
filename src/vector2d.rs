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