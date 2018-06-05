//! Aobench scene: 3 spheres and a plane using a random number generator

use geometry::{f32xN, Plane, Sphere, V3D};
use rand::{thread_rng, Rng};
use scene::Scene;

#[derive(Clone)]
pub struct Random {
    pub plane: Plane,
    pub spheres: [Sphere; 3],
}

impl Scene for Random {
    const NAO_SAMPLES: usize = 8;
    fn new() -> Self {
        let plane = Plane {
            p: V3D {
                x: 0.,
                y: -0.5,
                z: 0.,
            },
            n: V3D {
                x: 0.,
                y: 1.,
                z: 0.,
            },
        };
        let spheres = [
            Sphere {
                center: V3D {
                    x: -2.,
                    y: 0.,
                    z: -3.5,
                },
                radius: 0.5,
            },
            Sphere {
                center: V3D {
                    x: -0.5,
                    y: 0.,
                    z: -3.,
                },
                radius: 0.5,
            },
            Sphere {
                center: V3D {
                    x: 1.,
                    y: 0.,
                    z: -2.2,
                },
                radius: 0.5,
            },
        ];
        Self { plane, spheres }
    }
    fn rand(&mut self) -> f32 {
        let mut rng = thread_rng();
        rng.gen()
    }
    fn plane(&self) -> &Plane {
        &self.plane
    }
    fn spheres(&self) -> &[Sphere] {
        &self.spheres
    }
    fn rand_f32xN(&mut self) -> (f32xN, f32xN) {
        let mut rng = thread_rng();
        let r = [
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        ];
        (
            f32xN::new(r[0], r[2], r[4], r[6], r[8], r[10], r[12], r[14]),
            f32xN::new(r[1], r[3], r[5], r[7], r[9], r[11], r[13], r[15]),
        )
    }
}
