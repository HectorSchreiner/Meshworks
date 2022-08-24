use minifb::{Key, Window, WindowOptions};
use renderer::{Line, Renderer, Square};
use utils::{Color, Position, HEIGHT, PI, WIDTH};

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
        game(&mut renderer, &window);
        //renderer.clear(Color::DARK_GREY);
    }
}

fn game(renderer: &mut Renderer, window: &Window) {
    let mut fPlayerX = 5.0;
    let mut fPlayerY = 10.0;
    let mut fPlayerA = 0.0;

    let nMapHeight = 16;
    let nMapWidth = 16;

    let fFOV = PI / 4.0;
    let fDepth = 16.0;

    let mapString = String::from(
        "
    ################
    #..............#
    #..............#
    #..............#
    #..............#
    #..............#
    #..............#
    #..............#
    #..............#
    #..............#
    #..............#
    #..............#
    #..............#
    #..............#
    #..............#
    ################
    ",
    );

    let mapArray: Vec<_> = mapString.chars().collect();

    if window.is_key_pressed(Key::A, minifb::KeyRepeat::No) {
        fPlayerA -= 0.0001;
    }
    if window.is_key_pressed(Key::D, minifb::KeyRepeat::No) {
        fPlayerA += 0.0001;
    }

    for x in 0..WIDTH {
        // for hver column, udregn den projiterede vinkel til world space
        let mut fRayAngle = (fPlayerA - fFOV / 2.0) + (x as f32 / WIDTH as f32) * fFOV;

        let mut fDistanceToWall = 0.0;
        let mut bHitWall = false;

        let mut fEyeX = fRayAngle.sin();
        let mut fEyeY = fRayAngle.cos();

        while (bHitWall == false && fDistanceToWall < fDepth) {
            fDistanceToWall += 0.1;

            let mut nTestX = (fPlayerX + fEyeX * fDistanceToWall) as i32;
            let mut nTestY = (fPlayerY + fEyeY * fDistanceToWall) as i32;

            // test om ray er out of bounds
            if (nTestX < 0 || nTestX >= nMapWidth || nTestY < 0 || nTestY >= nMapHeight) {
                bHitWall = true; // bare sæt distance til max depth
                fDistanceToWall = fDepth;
            } else {
                {
                    if (mapArray[(nTestY * nMapWidth + nTestX) as usize] == '#') {
                        bHitWall = true;
                    }
                }
            }

            // calculate distance to ceiling and floor
            let mut nCeiling = (HEIGHT as f32 / 2.0) - (HEIGHT as f32 / fDistanceToWall);
            let mut nFloor = HEIGHT as f32 - nCeiling;

            for y in 0..HEIGHT {
                if y < nCeiling as usize {
                    &renderer.draw_pixel(
                        Position {
                            x: x as u32,
                            y: y as u32,
                        },
                        Color::BLACK,
                    );
                } else if y > nCeiling as usize && y <= nFloor as usize {
                    &renderer.draw_pixel(
                        Position {
                            x: x as u32,
                            y: y as u32,
                        },
                        Color::WHITE,
                    );
                } else {
                    &renderer.draw_pixel(
                        Position {
                            x: x as u32,
                            y: y as u32,
                        },
                        Color::DARK_GREY,
                    );
                }
            }
        }
    }
}
