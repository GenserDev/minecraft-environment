use crate::cube::Cube;
use crate::vector::Vec3;
use crate::material::*;
use std::fs;
use std::path::Path;

pub struct Scene {
    pub cubes: Vec<Cube>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { cubes: Vec::new() }
    }
    
    pub fn add_cube(&mut self, cube: Cube) {
        self.cubes.push(cube);
    }
    
    pub fn from_layers(layers_dir: &str) -> Self {
        let mut scene = Scene::new();
        
        let path = Path::new(layers_dir);
        
        if !path.exists() {
            println!("Carpeta de capas no encontrada. Creando escena de ejemplo...");
            return Self::create_example_scene();
        }
        
        let mut layer_files: Vec<_> = fs::read_dir(path)
            .expect("No se pudo leer el directorio de capas")
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "txt" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();
        
        // Ordenar numéricamente por el número en el nombre del archivo
        layer_files.sort_by_key(|path| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .and_then(|s| s.strip_prefix("layer"))
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0)
        });
        
        if layer_files.is_empty() {
            println!("No se encontraron archivos .txt en layers/. Creando escena de ejemplo...");
            return Self::create_example_scene();
        }
        
        for (layer_index, layer_file) in layer_files.iter().enumerate() {
            let file_name = layer_file.file_name().unwrap().to_str().unwrap();
            println!("Cargando capa {} desde: {}", layer_index + 1, file_name);
            let content = fs::read_to_string(layer_file)
                .expect("Error al leer archivo de capa");
            
            let lines: Vec<&str> = content.lines().collect();
            
            let mut blocks_in_layer = 0;
            for (z, line) in lines.iter().enumerate() {
                for (x, ch) in line.chars().enumerate() {
                    let block_type = ch.to_string();
                    let material = Self::get_material_from_char(&block_type);
                    
                    if let Some(mat) = material {
                        let position = Vec3::new(
                            x as f64,
                            layer_index as f64,
                            z as f64,
                        );
                        scene.add_cube(Cube::new(position, 1.0, mat));
                        blocks_in_layer += 1;
                    }
                }
            }
            println!("  -> {} bloques generados en esta capa", blocks_in_layer);
        }
        
        println!("Escena cargada con {} bloques", scene.cubes.len());
        scene
    }
    
    fn get_material_from_char(c: &str) -> Option<Material> {
        match c {
            "P" => {
                println!("  [P] Creando Piedra");
                Some(create_stone_material())
            },
            "A" => {
                println!("  [A] Creando Agua");
                Some(create_water_material())
            },
            "T" => {
                println!("  [T] Creando Tierra con césped");
                Some(create_dirt_material())
            },
            "M" => {
                println!("  [M] Creando Madera");
                Some(create_wood_material())
            },
            "H" => {
                println!("  [H] Creando Hojas");
                Some(create_leaves_material())
            },
            "C" => {
                println!("  [C] Creando Mineral de Carbón");
                Some(create_coal_ore_material())
            },
            "I" => {
                println!("  [I] Creando Mineral de Hierro");
                Some(create_iron_ore_material())
            },
            "D" => {
                println!("  [D] Creando Mineral de Diamante");
                Some(create_diamond_ore_material())
            },
            "X" | "_" | " " => None,
            _ => {
                println!("Advertencia: Caracter desconocido '{}'", c);
                None
            }
        }
    }
    
    fn create_example_scene() -> Self {
        let mut scene = Scene::new();
        
        println!("Creando escena de ejemplo simple...");
        
        for x in 0..5 {
            for z in 0..5 {
                scene.add_cube(Cube::new(
                    Vec3::new(x as f64, 0.0, z as f64),
                    1.0,
                    create_stone_material(),
                ));
            }
        }
        
        scene.add_cube(Cube::new(
            Vec3::new(2.0, 1.0, 2.0),
            1.0,
            create_water_material(),
        ));
        
        for y in 1..4 {
            scene.add_cube(Cube::new(
                Vec3::new(1.0, y as f64, 1.0),
                1.0,
                create_wood_material(),
            ));
        }
        
        for x in 0..3 {
            for z in 0..3 {
                scene.add_cube(Cube::new(
                    Vec3::new(x as f64, 4.0, z as f64),
                    1.0,
                    create_leaves_material(),
                ));
            }
        }
        
        println!("Escena de ejemplo creada con {} bloques", scene.cubes.len());
        scene
    }
}