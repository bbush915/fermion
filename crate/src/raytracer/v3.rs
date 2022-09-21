use std::ops;

use rand::random;
use serde::Deserialize;

use super::utils::random_in_range;

const EPSILON: f32 = 1e-8;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type P3 = V3;

impl V3 {
    pub fn len(&self) -> f32 {
        self.len2().sqrt()
    }

    pub fn len2(&self) -> f32 {
        V3::dot(self, self)
    }

    pub fn is_near_zero(&self) -> bool {
        (self.x.abs() < EPSILON) && (self.y.abs() < EPSILON) && (self.z.abs() < EPSILON)
    }

    pub fn dot(a: &V3, b: &V3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn hadamard(a: &V3, b: &V3) -> V3 {
        V3 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }

    pub fn cross(a: &V3, b: &V3) -> V3 {
        V3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn unit(&self) -> V3 {
        self.clone() / self.len()
    }

    pub fn reflect(a: V3, n: V3) -> V3 {
        a - (2. * V3::dot(&a, &n)) * n
    }

    pub fn refract(a: V3, n: V3, refrative_index_ratio: f32) -> V3 {
        let cosine_theta = V3::dot(&-a, &n).min(1.);

        let v_perpendicular = refrative_index_ratio * (a + cosine_theta * n);
        let v_parallel = (-(1. - v_perpendicular.len2()).abs().sqrt()) * n;

        v_perpendicular + v_parallel
    }

    pub fn random() -> V3 {
        V3 {
            x: random::<f32>(),
            y: random::<f32>(),
            z: random::<f32>(),
        }
    }

    pub fn random_in_range(minimum: f32, maximum: f32) -> V3 {
        V3 {
            x: random_in_range(minimum, maximum),
            y: random_in_range(minimum, maximum),
            z: random_in_range(minimum, maximum),
        }
    }

    pub fn random_in_disk(radius: f32) -> V3 {
        let mut result: V3;

        loop {
            result = V3 {
                x: random_in_range(-radius, radius),
                y: random_in_range(-radius, radius),
                z: 0.,
            };

            if result.len2() <= 1. {
                break;
            }
        }

        return result;
    }

    pub fn random_in_sphere(radius: f32) -> V3 {
        let mut result: V3;

        loop {
            result = V3::random_in_range(-radius, radius);

            if result.len2() <= 1. {
                break;
            }
        }

        return result;
    }
}

impl ops::Neg for V3 {
    type Output = V3;

    fn neg(self) -> V3 {
        V3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add<V3> for V3 {
    type Output = V3;

    fn add(self, rhs: V3) -> V3 {
        V3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<V3> for V3 {
    fn add_assign(&mut self, rhs: V3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<V3> for V3 {
    type Output = V3;

    fn sub(self, rhs: V3) -> V3 {
        V3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f32> for V3 {
    type Output = V3;

    fn mul(self, rhs: f32) -> V3 {
        V3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<V3> for f32 {
    type Output = V3;

    fn mul(self, rhs: V3) -> V3 {
        V3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Div<f32> for V3 {
    type Output = V3;

    fn div(self, rhs: f32) -> V3 {
        V3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<V3> for f32 {
    type Output = V3;

    fn div(self, rhs: V3) -> V3 {
        V3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}
