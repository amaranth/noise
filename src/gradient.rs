use std::num::cast;
use std::rand::Rng;
use super::{Best, Fast, Noise, Standard, Quality};
use super::util::{bilerp, lerp, scurve3, scurve5, trilerp};

pub struct GradientNoise {
    permutations: [uint, ..512],
    quality: Quality,
}

impl GradientNoise {
    pub fn new<T: Rng>(rand: &mut T, quality: Quality) -> GradientNoise {
        let mut r = GradientNoise {
            permutations: [0, ..512],
            quality: quality,
        };

        for i in range(0, 256) {
            r.permutations[i] = i;
        }

        for i in range(0, 256) {
            let pos = rand.gen_range(0, 256 - i) + i;
            r.permutations.swap(i, pos);
            r.permutations[i + 256] = r.permutations[i];
        }

        return r;
    }
}

impl Noise for GradientNoise {
    fn sample_1d<F: Float>(&self, x: F) -> F {
        // Find unit line that contains the point
        let X = (x.floor().to_int().unwrap() & 255) as uint;

        // Find relative coord of point in the line
        let rel_x = x - x.floor();

        // Compute fade curve
        let u = match self.quality {
            Fast => rel_x,
            Standard => scurve3(rel_x),
            Best => scurve5(rel_x),
        };

        // Add blended results from the 2 edges of the line
        let one = cast(1.0_f32).unwrap();
        let f0 = grad_1d(self.permutations[X],     rel_x);
        let f1 = grad_1d(self.permutations[X + 1], rel_x - one);

        return lerp(f0, f1, u);
    }

    fn sample_2d<F: Float>(&self, x: F, y: F) -> F {
        // Find unit square that contains the point
        let X = (x.floor().to_int().unwrap() & 255) as uint;
        let Y = (y.floor().to_int().unwrap() & 255) as uint;

        // Find relative x,y of point in the square
        let rel_x = x - x.floor();
        let rel_y = y - y.floor();

        // Compute fade curves
        let (u, v) = match self.quality {
            Fast => (
                rel_x,
                rel_y,
            ),
            Standard => (
                scurve3(rel_x),
                scurve3(rel_y),
            ),
            Best => (
                scurve5(rel_x),
                scurve5(rel_y),
            )
        };

        // Hash coordinates of 4 square corners
        let A  = self.permutations[X]     + Y;
        let B  = self.permutations[X + 1] + Y;

        // Add blended results from the 4 corners of the square
        let one = cast(1.0_f32).unwrap();
        let f00 = grad_2d(self.permutations[A],     rel_x,       rel_y);
        let f10 = grad_2d(self.permutations[B],     rel_x - one, rel_y);
        let f01 = grad_2d(self.permutations[A + 1], rel_x,       rel_y - one);
        let f11 = grad_2d(self.permutations[B + 1], rel_x - one, rel_y - one);

        return bilerp(f00, f10, f01, f11, u, v);
    }

    fn sample_3d<F: Float>(&self, x: F, y: F, z: F) -> F {
        // Find unit cube that contains the point
        let X = (x.floor().to_int().unwrap() & 255) as uint;
        let Y = (y.floor().to_int().unwrap() & 255) as uint;
        let Z = (z.floor().to_int().unwrap() & 255) as uint;

        // Find relative x,y,z of point in the cube
        let rel_x = x - x.floor();
        let rel_y = y - y.floor();
        let rel_z = z - z.floor();

        // Compute fade curves
        let (u, v, w) = match self.quality {
            Fast => (
                rel_x,
                rel_y,
                rel_z,
            ),
            Standard => (
                scurve3(rel_x),
                scurve3(rel_y),
                scurve3(rel_z),
            ),
            Best => (
                scurve5(rel_x),
                scurve5(rel_y),
                scurve5(rel_z),
            )
        };

        // Hash coordinates of 8 cube corners
        let A  = self.permutations[X]     + Y;
        let AA = self.permutations[A]     + Z;
        let AB = self.permutations[A + 1] + Z;
        let B  = self.permutations[X + 1] + Y;
        let BA = self.permutations[B]     + Z;
        let BB = self.permutations[B + 1] + Z;

        // Add blended results from the 8 corners of the cube
        let one = cast(1.0_f32).unwrap();
        let f000 = grad_3d(self.permutations[AA],     rel_x,       rel_y,       rel_z);
        let f100 = grad_3d(self.permutations[BA],     rel_x - one, rel_y,       rel_z);
        let f010 = grad_3d(self.permutations[AB],     rel_x,       rel_y - one, rel_z);
        let f110 = grad_3d(self.permutations[BB],     rel_x - one, rel_y - one, rel_z);
        let f001 = grad_3d(self.permutations[AA + 1], rel_x,       rel_y,       rel_z - one);
        let f101 = grad_3d(self.permutations[BA + 1], rel_x - one, rel_y,       rel_z - one);
        let f011 = grad_3d(self.permutations[AB + 1], rel_x,       rel_y - one, rel_z - one);
        let f111 = grad_3d(self.permutations[BB + 1], rel_x - one, rel_y - one, rel_z - one);

        return trilerp(f000, f100, f010, f110, f001, f101, f011, f111, u, v, w);
    }
}

fn grad_1d<F: Float>(hash: uint, x: F) -> F {
    if hash & 1 == 0  { x } else { -x }
}

fn grad_2d<F: Float>(hash: uint, x: F, y: F) -> F {
    // Convert low 3 bits of hash code into 4 gradient directions
    let h = hash & 3;
    let u = if h & 2 == 0 { x } else { -x };
    let v = if h & 1 == 0 { y } else { -y };

    return u + v;
}

fn grad_3d<F: Float>(hash: uint, x: F, y: F, z: F) -> F {
    // Convert low 4 bits of hash code into 12 gradient directions
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 { y } else if h == 12 || h == 14 { x } else { z };

    if h & 1 == 0 {
        if h & 2 == 0 {
            return u + v;
        } else {
            return u - v;
        }
    } else {
        if h & 2 == 0 {
            return -u + v;
        } else {
            return -u - v;
        }
    }
}
