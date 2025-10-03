use crate::vector::Vec3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone)]
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face: usize,
    pub u: f64,
    pub v: f64,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: f64, material: Material) -> Self {
        let half_size = size / 2.0;
        Cube {
            min: center - Vec3::new(half_size, half_size, half_size),
            max: center + Vec3::new(half_size, half_size, half_size),
            material,
        }
    }
    
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmin = t_min;
        let mut tmax = t_max;
        let mut hit_face = 0;
        
        // Intersección con planos X
        let inv_d = 1.0 / ray.direction.x;
        let mut t0 = (self.min.x - ray.origin.x) * inv_d;
        let mut t1 = (self.max.x - ray.origin.x) * inv_d;
        
        let face0 = if inv_d < 0.0 { 4 } else { 5 }; // East : West
        let face1 = if inv_d < 0.0 { 5 } else { 4 };
        
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        
        if t0 > tmin {
            tmin = t0;
            hit_face = face0;
        }
        if t1 < tmax {
            tmax = t1;
        }
        
        if tmin > tmax {
            return None;
        }
        
        // Intersección con planos Y
        let inv_d = 1.0 / ray.direction.y;
        let mut t0 = (self.min.y - ray.origin.y) * inv_d;
        let mut t1 = (self.max.y - ray.origin.y) * inv_d;
        
        let face0 = if inv_d < 0.0 { 0 } else { 1 }; // Top : Bottom
        let face1 = if inv_d < 0.0 { 1 } else { 0 };
        
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        
        if t0 > tmin {
            tmin = t0;
            hit_face = face0;
        }
        if t1 < tmax {
            tmax = t1;
        }
        
        if tmin > tmax {
            return None;
        }
        
        // Intersección con planos Z
        let inv_d = 1.0 / ray.direction.z;
        let mut t0 = (self.min.z - ray.origin.z) * inv_d;
        let mut t1 = (self.max.z - ray.origin.z) * inv_d;
        
        let face0 = if inv_d < 0.0 { 2 } else { 3 }; // North : South
        let face1 = if inv_d < 0.0 { 3 } else { 2 };
        
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        
        if t0 > tmin {
            tmin = t0;
            hit_face = face0;
        }
        if t1 < tmax {
            tmax = t1;
        }
        
        if tmin > tmax {
            return None;
        }
        
        let t = if tmin > t_min { tmin } else { tmax };
        
        if t < t_min || t > t_max {
            return None;
        }
        
        let point = ray.at(t);
        let normal = self.get_normal(hit_face);
        let (u, v) = self.get_uv(point, hit_face);
        
        Some(HitRecord {
            point,
            normal,
            t,
            face: hit_face,
            u,
            v,
            material: self.material.clone(),
        })
    }
    
    fn get_normal(&self, face: usize) -> Vec3 {
        match face {
            0 => Vec3::new(0.0, 1.0, 0.0),   // Top
            1 => Vec3::new(0.0, -1.0, 0.0),  // Bottom
            2 => Vec3::new(0.0, 0.0, -1.0),  // North
            3 => Vec3::new(0.0, 0.0, 1.0),   // South
            4 => Vec3::new(1.0, 0.0, 0.0),   // East
            5 => Vec3::new(-1.0, 0.0, 0.0),  // West
            _ => Vec3::new(0.0, 1.0, 0.0),
        }
    }
    
    fn get_uv(&self, point: Vec3, face: usize) -> (f64, f64) {
        let local = Vec3::new(
            (point.x - self.min.x) / (self.max.x - self.min.x),
            (point.y - self.min.y) / (self.max.y - self.min.y),
            (point.z - self.min.z) / (self.max.z - self.min.z),
        );
        
        match face {
            0 => (local.x, local.z),        // Top (Y+)
            1 => (local.x, 1.0 - local.z),  // Bottom (Y-)
            2 => (local.x, local.y),        // North (Z-)
            3 => (1.0 - local.x, local.y),  // South (Z+)
            4 => (local.z, local.y),        // East (X+)
            5 => (1.0 - local.z, local.y),  // West (X-)
            _ => (0.0, 0.0),
        }
    }
}