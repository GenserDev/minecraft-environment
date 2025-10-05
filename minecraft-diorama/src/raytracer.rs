use crate::scene::Scene;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::cube::HitRecord;
use image::{RgbImage, Rgb};
use rayon::prelude::*;

const MAX_DEPTH: u32 = 2;

pub fn render(scene: &Scene, camera: &Camera, width: u32, height: u32, samples: u32) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    let total_pixels = width * height;
    
    println!("Renderizando con paralelización Rayon...");
    
    // Convertir imagen a un vector de píxeles para procesamiento paralelo
    let pixels: Vec<(u32, u32, Rgb<u8>)> = (0..height)
        .into_par_iter()  // Paralelizar por filas
        .flat_map(|y| {
            (0..width).into_par_iter().map(move |x| {
                let mut color = [0.0, 0.0, 0.0];
                
                // Anti-aliasing con múltiples muestras
                for _ in 0..samples {
                    let u = (x as f64 + rand_float()) / (width - 1) as f64;
                    let v = ((height - 1 - y) as f64 + rand_float()) / (height - 1) as f64;
                    
                    let ray = camera.get_ray(u, v);
                    let sample_color = trace_ray(&ray, scene, 0);
                    
                    color[0] += sample_color[0];
                    color[1] += sample_color[1];
                    color[2] += sample_color[2];
                }
                
                // Promediar las muestras
                let scale = 1.0 / samples as f64;
                color[0] = (color[0] * scale).sqrt(); // Gamma correction
                color[1] = (color[1] * scale).sqrt();
                color[2] = (color[2] * scale).sqrt();
                
                let pixel = Rgb([
                    (color[0].clamp(0.0, 1.0) * 255.0) as u8,
                    (color[1].clamp(0.0, 1.0) * 255.0) as u8,
                    (color[2].clamp(0.0, 1.0) * 255.0) as u8,
                ]);
                
                (x, y, pixel)
            })
        })
        .collect();
    
    // Escribir los píxeles en la imagen
    for (x, y, pixel) in pixels {
        img.put_pixel(x, y, pixel);
    }
    
    println!("Renderizado completado!");
    img
}

pub fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> [f64; 3] {
    if depth >= MAX_DEPTH {
        return [0.0, 0.0, 0.0];
    }
    
    // Buscar la intersección más cercana
    let mut closest_hit: Option<HitRecord> = None;
    let mut closest_t = f64::INFINITY;
    
    for cube in &scene.cubes {
        if let Some(hit) = cube.intersect(ray, 0.001, closest_t) {
            closest_t = hit.t;
            closest_hit = Some(hit);
        }
    }
    
    if let Some(hit) = closest_hit {
        // Obtener el color de la textura
        let texture_color = hit.material.get_color(hit.face, hit.u, hit.v);
        let base_color = [
            texture_color[0] as f64 / 255.0,
            texture_color[1] as f64 / 255.0,
            texture_color[2] as f64 / 255.0,
        ];
        
        // Iluminación simple (luz direccional + ambiente)
        let light_dir = Vec3::new(0.5, 1.0, 0.3).normalize();
        let light_intensity = hit.normal.dot(&light_dir).max(0.0);
        let ambient = 0.4;
        let diffuse = 0.6 * light_intensity;
        let lighting = (ambient + diffuse).min(1.0);
        
        let mut final_color = [
            base_color[0] * lighting,
            base_color[1] * lighting,
            base_color[2] * lighting,
        ];
        
        // Solo calcular reflexión/refracción si la profundidad es baja
        if depth < 2 {
            // Reflexión (solo si es significativa)
            if hit.material.reflectivity > 0.3 {
                let reflected = ray.direction.reflect(&hit.normal);
                let reflected_ray = Ray::new(hit.point + hit.normal * 0.001, reflected);
                let reflected_color = trace_ray(&reflected_ray, scene, depth + 1);
                
                let ref_amount = hit.material.reflectivity * 0.5;
                final_color[0] = final_color[0] * (1.0 - ref_amount) 
                               + reflected_color[0] * ref_amount;
                final_color[1] = final_color[1] * (1.0 - ref_amount) 
                               + reflected_color[1] * ref_amount;
                final_color[2] = final_color[2] * (1.0 - ref_amount) 
                               + reflected_color[2] * ref_amount;
            }
            
            // Refracción (solo si es muy transparente)
            if hit.material.transparency > 0.5 {
                let eta_ratio = if hit.normal.dot(&ray.direction) < 0.0 {
                    1.0 / hit.material.refractive_index
                } else {
                    hit.material.refractive_index
                };
                
                let normal = if hit.normal.dot(&ray.direction) < 0.0 {
                    hit.normal
                } else {
                    -hit.normal
                };
                
                if let Some(refracted) = ray.direction.refract(&normal, eta_ratio) {
                    let refracted_ray = Ray::new(hit.point - normal * 0.001, refracted);
                    let refracted_color = trace_ray(&refracted_ray, scene, depth + 1);
                    
                    let trans_amount = hit.material.transparency * 0.5;
                    final_color[0] = final_color[0] * (1.0 - trans_amount) 
                                   + refracted_color[0] * trans_amount;
                    final_color[1] = final_color[1] * (1.0 - trans_amount) 
                                   + refracted_color[1] * trans_amount;
                    final_color[2] = final_color[2] * (1.0 - trans_amount) 
                                   + refracted_color[2] * trans_amount;
                }
            }
        }
        
        final_color
    } else {
        // Skybox - cielo degradado
        skybox_color(&ray.direction)
    }
}

fn skybox_color(direction: &Vec3) -> [f64; 3] {
    let t = 0.5 * (direction.normalize().y + 1.0);
    
    // Color del cielo: azul claro arriba, blanco abajo
    let color1 = [0.5, 0.7, 1.0]; // Azul cielo
    let color2 = [1.0, 1.0, 1.0]; // Blanco
    
    [
        color1[0] * t + color2[0] * (1.0 - t),
        color1[1] * t + color2[1] * (1.0 - t),
        color1[2] * t + color2[2] * (1.0 - t),
    ]
}

// Función auxiliar para generar números aleatorios simples
fn rand_float() -> f64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};
    
    let random_state = RandomState::new();
    let mut hasher = random_state.build_hasher();
    
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .hash(&mut hasher);
    
    (hasher.finish() % 10000) as f64 / 10000.0
}