mod vector;
mod ray;
mod camera;
mod material;
mod cube;
mod scene;
mod raytracer;

use camera::Camera;
use scene::Scene;
use vector::Vec3;
use winit::event::{Event, WindowEvent, ElementState, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use pixels::{Pixels, SurfaceTexture};
use std::time::Instant;
use rayon::prelude::*;

struct CameraController {
    position: Vec3,
    yaw: f64,
    pitch: f64,
    speed: f64,
    sensitivity: f64,
    
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl CameraController {
    fn new(position: Vec3) -> Self {
        CameraController {
            position,
            yaw: -135.0,
            pitch: -30.0,
            speed: 10.0,
            sensitivity: 1.0,
            forward: false,
            backward: false,
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }
    
    fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) {
        let pressed = state == ElementState::Pressed;
        match key {
            VirtualKeyCode::W => self.forward = pressed,
            VirtualKeyCode::S => self.backward = pressed,
            VirtualKeyCode::A => self.left = pressed,
            VirtualKeyCode::D => self.right = pressed,
            VirtualKeyCode::Space => self.up = pressed,
            VirtualKeyCode::LShift => self.down = pressed,
            _ => {}
        }
    }
    
    fn update(&mut self, delta_time: f64) {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();
        
        let forward = Vec3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        ).normalize();
        
        let right = Vec3::new(
            (yaw_rad - std::f64::consts::PI / 2.0).cos(),
            0.0,
            (yaw_rad - std::f64::consts::PI / 2.0).sin(),
        ).normalize();
        
        let up = Vec3::new(0.0, 1.0, 0.0);
        
        let speed = self.speed * delta_time;
        
        if self.forward {
            self.position = self.position + forward * speed;
        }
        if self.backward {
            self.position = self.position - forward * speed;
        }
        if self.right {
            self.position = self.position - right * speed;
        }
        if self.left {
            self.position = self.position + right * speed;
        }
        if self.up {
            self.position = self.position + up * speed;
        }
        if self.down {
            self.position = self.position - up * speed;
        }
    }
    
    fn rotate(&mut self, delta_x: f64, delta_y: f64) {
        self.yaw += delta_x * self.sensitivity;
        self.pitch -= delta_y * self.sensitivity;
        self.pitch = self.pitch.clamp(-89.0, 89.0);
    }
    
    fn get_camera(&self, aspect_ratio: f64) -> Camera {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();
        
        let look_at = self.position + Vec3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        );
        
        Camera::new(
            self.position,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            aspect_ratio,
        )
    }
}

fn main() {
    println!("Iniciando diorama Minecraft interactivo...");
    println!("\nControles:");
    println!("  W/A/S/D - Mover cámara");
    println!("  Space - Subir");
    println!("  Shift - Bajar");
    println!("  Mouse - Rotar cámara (click izquierdo y arrastra)");
    println!("  ESC - Salir");
    
    // Cargar la escena
    println!("\nCargando escena...");
    let scene = Scene::from_layers("layers/");
    println!("Bloques cargados: {}", scene.cubes.len());
    
    // Configuración de ventana
    let window_width = 1280u32;
    let window_height = 720u32;
    
    // Resolución de renderizado (ajusta según tu hardware)
    let render_width = 320u32;
    let render_height = 180u32;
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Minecraft Diorama - Raytracing Interactivo (Rayon)")
        .with_inner_size(winit::dpi::LogicalSize::new(window_width, window_height))
        .build(&event_loop)
        .unwrap();
    
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(render_width, render_height, surface_texture).unwrap();
    
    let mut controller = CameraController::new(Vec3::new(6.0, 3.5, 6.0));
    let mut last_frame = Instant::now();
    let mut mouse_grabbed = false;
    let mut last_mouse_pos: Option<(f64, f64)> = None;
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();
    
    println!("\n¡Ventana abierta! Usa el mouse y teclado para navegar.");
    println!("OPTIMIZACIÓN: Usando paralelización Rayon para mejor rendimiento");
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        if keycode == VirtualKeyCode::Escape {
                            *control_flow = ControlFlow::Exit;
                        } else {
                            controller.process_keyboard(keycode, input.state);
                        }
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == winit::event::MouseButton::Left {
                        mouse_grabbed = state == ElementState::Pressed;
                        if !mouse_grabbed {
                            last_mouse_pos = None;
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if mouse_grabbed {
                        if let Some((last_x, last_y)) = last_mouse_pos {
                            let delta_x = position.x - last_x;
                            let delta_y = position.y - last_y;
                            controller.rotate(delta_x, delta_y);
                        }
                        last_mouse_pos = Some((position.x, position.y));
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta_time = now.duration_since(last_frame).as_secs_f64();
                last_frame = now;
                
                controller.update(delta_time);
                
                // Contador de FPS
                frame_count += 1;
                if fps_timer.elapsed().as_secs() >= 1 {
                    println!("FPS: {}", frame_count);
                    frame_count = 0;
                    fps_timer = Instant::now();
                }
                
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let aspect_ratio = render_width as f64 / render_height as f64;
                let camera = controller.get_camera(aspect_ratio);
                
                render_to_pixels_parallel(&scene, &camera, pixels.frame_mut(), render_width, render_height);
                
                if let Err(err) = pixels.render() {
                    eprintln!("Error al renderizar: {}", err);
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}

// Versión paralela del renderizado en tiempo real
fn render_to_pixels_parallel(scene: &Scene, camera: &Camera, frame: &mut [u8], width: u32, height: u32) {
    let pixels: Vec<(usize, [u8; 4])> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            (0..width).into_par_iter().map(move |x| {
                let u = x as f64 / (width - 1) as f64;
                let v = ((height - 1 - y) as f64) / (height - 1) as f64;
                
                let ray = camera.get_ray(u, v);
                let color = raytracer::trace_ray(&ray, scene, 0);
                
                let r = (color[0].clamp(0.0, 1.0).sqrt() * 255.0) as u8;
                let g = (color[1].clamp(0.0, 1.0).sqrt() * 255.0) as u8;
                let b = (color[2].clamp(0.0, 1.0).sqrt() * 255.0) as u8;
                
                let idx = ((y * width + x) * 4) as usize;
                (idx, [r, g, b, 255])
            })
        })
        .collect();
    
    // Escribir los píxeles en el frame
    for (idx, pixel) in pixels {
        frame[idx..idx + 4].copy_from_slice(&pixel);
    }
}