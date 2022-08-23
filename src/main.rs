use mafs::{mesh, triangle, vec3d};
use minifb::{Key, Window, WindowOptions};
use renderer::{Line, Renderer, Square};
use utils::{Color, Position, HEIGHT, PI, WIDTH};

mod mafs;
mod renderer;
mod utils;

fn main() {
    let mut renderer: Renderer = Renderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    let mut window = Window::new(
        "Meshworks 3D Graphics Engine",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
        onCreate();
        renderer.clear(Color::DARK_GREY);
    }
}

fn onCreate() {
    let meshCube: mesh;

    meshCube.tris = {
        
    };
}
