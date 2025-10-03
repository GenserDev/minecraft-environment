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
        
        // Leer archivos de capas
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
        
        layer_files.sort();
        
        if layer_files.is_empty() {
            println!("No se encontraron archivos .txt en layers/. Creando escena de ejemplo...");
            return Self::create_example_scene();
        }
        
        for (layer_index, layer_file) in layer_files.iter().enumerate() {
            println!("Cargando capa: {:?}", layer_file);
            let content = fs::read_to_string(layer_file)
                .expect("Error al leer archivo de capa");
            
            let lines: Vec<&str> = content.lines().collect();
            
            for (z, line) in lines.iter().enumerate() {
                let blocks: Vec<&str> = line.split_whitespace().collect();
                
                for (x, block_type) in blocks.iter().enumerate() {
                    let material = Self::get_material_from_char(block_type);
                    
                    if let Some(mat) = material {
                        let position = Vec3::new(
                            x as f64,
                            layer_index as f64,
                            z as f64,
                        );
                        scene.add_cube(Cube::new(position, 1.0, mat));
                    }
                }
            }
        }
        
        scene
    }
    
    fn get_material_from_char(c: &str) -> Option<Material> {
        match c {
            "G" => Some(create_grass_material()),
            "D" => Some(create_dirt_material()),
            "S" => Some(create_stone_material()),
            "C" => Some(create_coal_ore_material()),
            "I" => Some(create_iron_ore_material()),
            "M" => Some(create_diamond_ore_material()),
            "L" => Some(create_glass_material()),
            "W" => Some(create_wood_material()),
            "F" => Some(create_leaves_material()),
            "A" => Some(create_water_material()),
            "R" => Some(create_creeper_material()),
            "_" => None, // Aire / vacío
            _ => None,
        }
    }
    
    fn create_example_scene() -> Self {
        let mut scene = Scene::new();
        
        let size_x = 12;
        let size_z = 12;
        
        // ===== ESTRUCTURA DE CUEVA =====
        
        // Capa 0-1: Base de piedra sólida
        for y in 0..2 {
            for x in 0..size_x {
                for z in 0..size_z {
                    scene.add_cube(Cube::new(
                        Vec3::new(x as f64, y as f64, z as f64),
                        1.0,
                        create_stone_material(),
                    ));
                }
            }
        }
        
        // Capa 2-4: Paredes de la cueva (hueca en el centro)
        for y in 2..5 {
            for x in 0..size_x {
                for z in 0..size_z {
                    // Solo crear bloques en los bordes (paredes)
                    let is_wall = x == 0 || x == size_x - 1 || z == 0 || z == size_z - 1;
                    
                    if is_wall {
                        let material = if (x + z + y) % 4 == 0 {
                            create_coal_ore_material()
                        } else if (x + z + y) % 7 == 0 {
                            create_iron_ore_material()
                        } else {
                            create_stone_material()
                        };
                        
                        scene.add_cube(Cube::new(
                            Vec3::new(x as f64, y as f64, z as f64),
                            1.0,
                            material,
                        ));
                    }
                }
            }
        }
        
        // Agregar algunas columnas/pilares dentro de la cueva
        let pillars = vec![
            (3, 3), (8, 3), (3, 8), (8, 8), (5, 5)
        ];
        
        for (px, pz) in pillars {
            for y in 2..5 {
                scene.add_cube(Cube::new(
                    Vec3::new(px as f64, y as f64, pz as f64),
                    1.0,
                    create_stone_material(),
                ));
            }
        }
        
        // Agregar minerales especiales en paredes
        scene.add_cube(Cube::new(
            Vec3::new(1.0, 3.0, 5.0),
            1.0,
            create_diamond_ore_material(),
        ));
        scene.add_cube(Cube::new(
            Vec3::new(10.0, 2.0, 6.0),
            1.0,
            create_diamond_ore_material(),
        ));
        
        // Agua en una esquina de la cueva
        for x in 1..3 {
            for z in 1..3 {
                scene.add_cube(Cube::new(
                    Vec3::new(x as f64, 2.0, z as f64),
                    1.0,
                    create_water_material(),
                ));
            }
        }
        
        // ===== TECHO DE LA CUEVA (TIERRA Y CÉSPED) =====
        
        // Capa 5: Tierra (techo de la cueva)
        for x in 0..size_x {
            for z in 0..size_z {
                scene.add_cube(Cube::new(
                    Vec3::new(x as f64, 5.0, z as f64),
                    1.0,
                    create_dirt_material(),
                ));
            }
        }
        
        // Capa 6: Césped en la superficie
        for x in 0..size_x {
            for z in 0..size_z {
                scene.add_cube(Cube::new(
                    Vec3::new(x as f64, 6.0, z as f64),
                    1.0,
                    create_grass_material(),
                ));
            }
        }
        
        // ===== ÁRBOL EN LA SUPERFICIE =====
        
        let tree_x = 6.0;
        let tree_z = 6.0;
        
        // Tronco del árbol (sobre el césped)
        for y in 7..10 {
            scene.add_cube(Cube::new(
                Vec3::new(tree_x, y as f64, tree_z),
                1.0,
                create_wood_material(),
            ));
        }
        
        // Copa del árbol - Capa inferior (y=9)
        for x in 4..9 {
            for z in 4..9 {
                if !(x == 6 && z == 6) {  // No poner hoja donde está el tronco
                    scene.add_cube(Cube::new(
                        Vec3::new(x as f64, 9.0, z as f64),
                        1.0,
                        create_leaves_material(),
                    ));
                }
            }
        }
        
        // Copa del árbol - Capa media (y=10)
        for x in 4..9 {
            for z in 4..9 {
                scene.add_cube(Cube::new(
                    Vec3::new(x as f64, 10.0, z as f64),
                    1.0,
                    create_leaves_material(),
                ));
            }
        }
        
        // Copa del árbol - Capa superior (y=11) más pequeña
        for x in 5..8 {
            for z in 5..8 {
                scene.add_cube(Cube::new(
                    Vec3::new(x as f64, 11.0, z as f64),
                    1.0,
                    create_leaves_material(),
                ));
            }
        }
        
        // Copa del árbol - Punta (y=12)
        scene.add_cube(Cube::new(
            Vec3::new(tree_x, 12.0, tree_z),
            1.0,
            create_leaves_material(),
        ));
        
        // ===== DECORACIÓN EXTRA =====
        
        // Algunos bloques de vidrio como "ventanas" en las paredes
        scene.add_cube(Cube::new(
            Vec3::new(0.0, 3.0, 5.0),
            1.0,
            create_glass_material(),
        ));
        scene.add_cube(Cube::new(
            Vec3::new(11.0, 3.0, 6.0),
            1.0,
            create_glass_material(),
        ));
        
        // Un Creeper dentro de la cueva
        let creeper_x = 9.0;
        let creeper_z = 4.0;
        
        // Cuerpo del Creeper
        scene.add_cube(Cube::new(
            Vec3::new(creeper_x, 2.0, creeper_z),
            0.8,
            create_creeper_material(),
        ));
        scene.add_cube(Cube::new(
            Vec3::new(creeper_x, 3.0, creeper_z),
            0.8,
            create_creeper_material(),
        ));
        
        // Cabeza del Creeper
        scene.add_cube(Cube::new(
            Vec3::new(creeper_x, 4.0, creeper_z),
            0.9,
            create_creeper_material(),
        ));
        
        // Patas del Creeper (4 patas pequeñas)
        let leg_size = 0.3;
        scene.add_cube(Cube::new(
            Vec3::new(creeper_x - 0.2, 1.65, creeper_z - 0.2),
            leg_size,
            create_creeper_material(),
        ));
        scene.add_cube(Cube::new(
            Vec3::new(creeper_x + 0.2, 1.65, creeper_z - 0.2),
            leg_size,
            create_creeper_material(),
        ));
        scene.add_cube(Cube::new(
            Vec3::new(creeper_x - 0.2, 1.65, creeper_z + 0.2),
            leg_size,
            create_creeper_material(),
        ));
        scene.add_cube(Cube::new(
            Vec3::new(creeper_x + 0.2, 1.65, creeper_z + 0.2),
            leg_size,
            create_creeper_material(),
        ));
        
        println!("Escena tipo cueva creada con {} bloques", scene.cubes.len());
        println!("Estructura: Cueva subterránea (y=2-4) con césped y árbol arriba (y=6-12)");
        
        scene
    }
}