//! Intersection of a ray with a sphere.

use geometry::{f32xN, Dot, Ray, RayxN, Selectable, Sphere};
use intersection::{Intersect, Isect, IsectxN};

// Scalar ray-sphere intersection
impl Intersect<Sphere> for Ray {
    type Isect = Isect;
    #[inline(always)]
    fn intersect(&self, sphere: &Sphere, mut isect: Isect) -> Isect {
        let ray = self;
        let rs = ray.origin - sphere.center;

        let b = rs.dot(ray.dir);
        let c = rs.dot(rs) - sphere.radius * sphere.radius;
        let d = b * b - c;

        if d > 0. {
            let t = -b - d.sqrt();

            if t > 0. && t < isect.t {
                isect.t = t;
                isect.hit = true;
                isect.p = ray.origin + t * ray.dir;
                isect.n = (isect.p - sphere.center).normalized();
            }
        }

        isect
    }
}

// Vector ray-sphere intersection for a packet of rays
impl Intersect<Sphere> for RayxN {
    type Isect = IsectxN;
    #[inline(always)]
    fn intersect(&self, sphere: &Sphere, mut isect: IsectxN) -> IsectxN {
        let ray = self;
        let rs = ray.origin - sphere.center;

        let b = rs.dot(ray.dir);
        let c = rs.dot(rs) - sphere.radius * sphere.radius;
        let d = b * b - c;

        let old_isect = isect;

        let m = d.gt(f32xN::splat(0.));
        if m.any() {
            let t = m.sel(-b - d.sqrt(), isect.t);
            let m = m & t.gt(f32xN::splat(0.)) & t.lt(isect.t);

            isect.t = m.sel(t, isect.t);
            isect.hit = m | isect.hit;
            isect.p = m.sel(ray.origin + t * ray.dir, isect.p);
            isect.n = m.sel((isect.p - sphere.center).normalized(), isect.n);
        }

        debug_assert!({
            // Check that the vector and the scalar version produce the same results
            // for the same inputs in debug builds
            for i in 0..f32xN::lanes() {
                let old_isect_i = old_isect.get(i);
                let ray_i = self.get(i);
                let isect_i = ray_i.intersect(sphere, old_isect_i);
                assert_eq!(isect_i, isect.get(i), "\n\nsphere: {:?}\n\nold_isect: {:?}\n\nrays: {:?}\n\ni: {:?}\nold_isect_i: {:?}\nray_i: {:?}\nisect_i: {:?}\n\n", sphere, old_isect, self, i, old_isect_i, ray_i, isect_i);
            }
            true
        });

        isect
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geometry::{m32xN, V3D, V3DxN};

    #[test]
    fn sanity() {
        let sphere = Sphere {
            center: V3D {
                x: 0.,
                y: 0.,
                z: -10.,
            },
            radius: 1.,
        };

        let ray_hit = Ray {
            origin: V3D::new(),
            dir: V3D {
                x: 0.01,
                y: 0.01,
                z: -1.,
            },
        };
        let ray_miss = Ray {
            origin: V3D::new(),
            dir: V3D {
                x: 0.,
                y: 0.,
                z: 1.,
            },
        };

        let isect_hit = ray_hit.intersect(&sphere, Isect::new());
        assert!(isect_hit.hit);
        let isect_miss = ray_miss.intersect(&sphere, Isect::new());
        assert!(!isect_miss.hit);

        // hit, miss, hit, miss
        #[cfg(feature = "256bit")]
        let z_val = f32xN::new(-1., 1., -1., 1., -1., 1., -1., 1.);
        #[cfg(not(feature = "256bit"))]
        let z_val = f32xN::new(-1., 1., -1., 1.);

        let rays = RayxN {
            origin: V3DxN::new(),
            dir: V3DxN {
                x: f32xN::splat(0.01),
                y: f32xN::splat(0.01),
                z: z_val,
            },
        };

        let isectxN = rays.intersect(&sphere, IsectxN::new());

        #[cfg(feature = "256bit")]
        let expected = m32xN::new(true, false, true, false, true, false, true, false);
        #[cfg(not(feature = "256bit"))]
        let expected = m32xN::new(true, false, true, false);

        assert_eq!(
            isectxN.hit,
            expected
        );

        assert_eq!(isect_hit.t, isectxN.t.extract(0));
        assert_eq!(isect_hit.t, isectxN.t.extract(2));
        assert_eq!(isect_miss.t, isectxN.t.extract(1));
        assert_eq!(isect_miss.t, isectxN.t.extract(3));

        assert_eq!(isect_hit.p.x, isectxN.p.x.extract(0));
        assert_eq!(isect_hit.p.y, isectxN.p.y.extract(0));
        assert_eq!(isect_hit.p.z, isectxN.p.z.extract(0));

        assert_eq!(isect_hit.p.x, isectxN.p.x.extract(2));
        assert_eq!(isect_hit.p.y, isectxN.p.y.extract(2));
        assert_eq!(isect_hit.p.z, isectxN.p.z.extract(2));

        assert_eq!(isect_miss.p.x, isectxN.p.x.extract(1));
        assert_eq!(isect_miss.p.y, isectxN.p.y.extract(1));
        assert_eq!(isect_miss.p.z, isectxN.p.z.extract(1));

        assert_eq!(isect_miss.p.x, isectxN.p.x.extract(3));
        assert_eq!(isect_miss.p.y, isectxN.p.y.extract(3));
        assert_eq!(isect_miss.p.z, isectxN.p.z.extract(3));

        assert_eq!(isect_hit.n.x, isectxN.n.x.extract(0));
        assert_eq!(isect_hit.n.y, isectxN.n.y.extract(0));
        assert_eq!(isect_hit.n.z, isectxN.n.z.extract(0));

        assert_eq!(isect_hit.n.x, isectxN.n.x.extract(2));
        assert_eq!(isect_hit.n.y, isectxN.n.y.extract(2));
        assert_eq!(isect_hit.n.z, isectxN.n.z.extract(2));

        assert_eq!(isect_miss.n.x, isectxN.n.x.extract(1));
        assert_eq!(isect_miss.n.y, isectxN.n.y.extract(1));
        assert_eq!(isect_miss.n.z, isectxN.n.z.extract(1));

        assert_eq!(isect_miss.n.x, isectxN.n.x.extract(3));
        assert_eq!(isect_miss.n.y, isectxN.n.y.extract(3));
        assert_eq!(isect_miss.n.z, isectxN.n.z.extract(3));
    }
}
