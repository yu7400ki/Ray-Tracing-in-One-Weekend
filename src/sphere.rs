use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let rd = r.direction();
        let a = rd.length_squared();
        let half_b = oc.dot(rd);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let root = discriminant.sqrt();
        let t = (-half_b - root) / a;
        if t_min < t && t < t_max {
            let p = r.at(t);
            let outward_normal = (p - self.center) / self.radius;
            let mut rec = HitRecord {
                p,
                normal: outward_normal,
                t,
                front_face: true,
            };
            rec.set_face_normal(r, outward_normal);
            return Some(rec);
        }
        let t = (-half_b + root) / a;
        if t_min < t && t < t_max {
            let p = r.at(t);
            let outward_normal = (p - self.center) / self.radius;
            let mut rec = HitRecord {
                p,
                normal: outward_normal,
                t,
                front_face: true,
            };
            rec.set_face_normal(r, outward_normal);
            return Some(rec);
        }
        None
    }
}
