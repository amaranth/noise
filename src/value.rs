use std::num::cast;
use super::IntegerNoise;
use super::{Best, Fast, Noise, Standard, Quality};
use super::util::{bilerp, lerp, scurve3, scurve5, trilerp};

#[deriving(Clone)]
pub struct ValueNoise {
    source: IntegerNoise,
    quality: Quality,
}

impl ValueNoise {
    pub fn new(seed: int, quality: Quality) -> ValueNoise {
        ValueNoise {
            source: IntegerNoise::new(seed),
            quality: quality,
        }
    }
}

impl Noise for ValueNoise {
    fn sample_1d<F: Float>(&self, x: F) -> F {
        let u = match self.quality {
            Fast => x - x.floor(),
            Standard => scurve3(x - x.floor()),
            Best => scurve5(x - x.floor()),
        };

        let one = cast(1.0_f32).unwrap();
        let f0 = self.source.sample_1d(x);
        let f1 = self.source.sample_1d(x + one);

        return lerp(f0, f1, u);
    }

    fn sample_2d<F: Float>(&self, x: F, y: F) -> F {
        let (u, v) = match self.quality {
            Fast => (
                x - x.floor(),
                y - y.floor(),
            ),
            Standard => (
                scurve3(x - x.floor()),
                scurve3(y - y.floor()),
            ),
            Best => (
                scurve5(x - x.floor()),
                scurve5(y - y.floor()),
            )
        };

        let one = cast(1.0_f32).unwrap();
        let f00 = self.source.sample_2d(x,       y);
        let f10 = self.source.sample_2d(x + one, y);
        let f01 = self.source.sample_2d(x,       y + one);
        let f11 = self.source.sample_2d(x + one, y + one);

        return bilerp(f00, f10, f01, f11, u, v);
    }

    fn sample_3d<F: Float>(&self, x: F, y: F, z: F) -> F {
        let (u, v, w) = match self.quality {
            Fast => (
                x - x.floor(),
                y - y.floor(),
                z - z.floor(),
            ),
            Standard => (
                scurve3(x - x.floor()),
                scurve3(y - y.floor()),
                scurve3(z - z.floor()),
            ),
            Best => (
                scurve5(x - x.floor()),
                scurve5(y - y.floor()),
                scurve5(z - z.floor()),
            )
        };

        let one = cast(1.0_f32).unwrap();
        let f000 = self.source.sample_3d(x,       y,       z);
        let f100 = self.source.sample_3d(x + one, y,       z);
        let f010 = self.source.sample_3d(x,       y + one, z);
        let f110 = self.source.sample_3d(x + one, y + one, z);
        let f001 = self.source.sample_3d(x,       y,       z + one);
        let f101 = self.source.sample_3d(x + one, y,       z + one);
        let f011 = self.source.sample_3d(x,       y + one, z + one);
        let f111 = self.source.sample_3d(x + one, y + one, z + one);

        return trilerp(f000, f100, f010, f110, f001, f101, f011, f111, u, v, w);
    }
}
