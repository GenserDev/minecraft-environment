use image::{DynamicImage, GenericImageView};
use std::path::Path;

#[derive(Clone)]
pub struct Texture {
    image: DynamicImage,
}

impl Texture {
    pub fn load(path: &str) -> Option<Self> {
        if let Ok(img) = image::open(Path::new(path)) {
            println!("✓ Textura cargada: {}", path);
            Some(Texture { image: img })
        } else {
            println!("✗ Advertencia: No se pudo cargar textura: {}", path);
            None
        }
    }
    
    pub fn get_color(&self, u: f64, v: f64) -> [u8; 3] {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Invertir V
        
        let width = self.image.width();
        let height = self.image.height();
        
        let x = ((u * width as f64) as u32).min(width - 1);
        let y = ((v * height as f64) as u32).min(height - 1);
        
        let pixel = self.image.get_pixel(x, y);
        [pixel[0], pixel[1], pixel[2]]
    }
}

#[derive(Clone)]
pub struct Material {
    pub textures: [Option<Texture>; 6], // Top, Bottom, North, South, East, West
    pub reflectivity: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub base_color: [u8; 3],
}

impl Material {
    pub fn new(base_color: [u8; 3]) -> Self {
        Material {
            textures: [None, None, None, None, None, None],
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            base_color,
        }
    }
    
    pub fn with_texture(mut self, face: usize, texture_path: &str) -> Self {
        if face < 6 {
            self.textures[face] = Texture::load(texture_path);
        }
        self
    }
    
    pub fn with_all_textures(mut self, texture_path: &str) -> Self {
        let texture = Texture::load(texture_path);
        for i in 0..6 {
            self.textures[i] = texture.clone();
        }
        self
    }
    
    pub fn with_reflectivity(mut self, reflectivity: f64) -> Self {
        self.reflectivity = reflectivity;
        self
    }
    
    pub fn with_transparency(mut self, transparency: f64, refractive_index: f64) -> Self {
        self.transparency = transparency;
        self.refractive_index = refractive_index;
        self
    }
    
    pub fn get_color(&self, face: usize, u: f64, v: f64) -> [u8; 3] {
        if face < 6 {
            if let Some(ref texture) = self.textures[face] {
                return texture.get_color(u, v);
            }
        }
        self.base_color
    }
}

// ===== MATERIALES QUE USAS EN TU PROYECTO =====

// P - Piedra (Stone)
pub fn create_stone_material() -> Material {
    Material::new([128, 128, 128])
        .with_all_textures("textures/stone.png")
}

// T - Tierra (Dirt)
pub fn create_dirt_material() -> Material {
    Material::new([139, 90, 43])
        .with_all_textures("textures/dirt.png")
}

// M - Madera (Wood)
pub fn create_wood_material() -> Material {
    Material::new([139, 90, 43])
        .with_all_textures("textures/wood.png")
}

// H - Hojas (Leaves)
pub fn create_leaves_material() -> Material {
    Material::new([34, 139, 34])
        .with_all_textures("textures/leaves.png")
        .with_transparency(0.2, 1.0)
}

// A - Agua (Water)
pub fn create_water_material() -> Material {
    Material::new([30, 70, 200])
        .with_all_textures("textures/water.png")
        .with_transparency(0.7, 1.33)
        .with_reflectivity(0.2)
}

// ===== MATERIALES ADICIONALES (por si los necesitas más adelante) =====

pub fn create_grass_material() -> Material {
    Material::new([34, 139, 34])
        .with_texture(0, "textures/grass_top.png")    // Top
        .with_texture(1, "textures/dirt.png")         // Bottom
        .with_texture(2, "textures/grass_side.png")   // North
        .with_texture(3, "textures/grass_side.png")   // South
        .with_texture(4, "textures/grass_side.png")   // East
        .with_texture(5, "textures/grass_side.png")   // West
}

pub fn create_coal_ore_material() -> Material {
    Material::new([64, 64, 64])
        .with_all_textures("textures/coal_ore.png")
}

pub fn create_iron_ore_material() -> Material {
    Material::new([188, 152, 98])
        .with_all_textures("textures/iron_ore.png")
}

pub fn create_diamond_ore_material() -> Material {
    Material::new([100, 200, 200])
        .with_all_textures("textures/diamond_ore.png")
        .with_reflectivity(0.3)
}