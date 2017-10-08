extern crate sdl2;

use sdl2::event::{Event,WindowEvent};
use sdl2::keyboard::Keycode;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut timer = ctx.timer().unwrap();

    let mut window = match video_ctx.window("openrp", 1280, 720).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let mut events = ctx.event_pump().unwrap();
    'event : loop {
        for event in events.wait_iter() {
            match event {
                Event::Quit{..} => break 'event,
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'event
                    }
                }
                _ => continue
            }
        }
    }
}
