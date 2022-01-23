mod scene;

extern crate hlua;
extern crate sdl2;

use hlua::Lua;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use cgmath::Vector3;
use sdl2::rect::Point;
use crate::scene::compute_scene;


fn launch_vm() {
    let mut lua = Lua::new();     // mutable is mandatory

    lua.set("x", 2);
    lua.set("x", 3);
    lua.execute::<()>("x = x + 1").unwrap();
    let x: i32 = lua.get("x").unwrap();  // x is equal to 3

}

pub fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 1920, 1080)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // canvas.set_draw_color(Color::RGB(255, 0, 0));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut inc = Vector3::new(0., 0., 0.);

    let value = 0.2;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                | Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => inc.y += value,
                | Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => inc.y -= value,
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => inc.x += value,
                | Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => inc.x -= value,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        compute_scene(&mut canvas, &inc);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}