extern crate sdl2;

#[macro_use]
mod phi;
mod views;

use sdl2::pixels::Color;
use phi::{Events, Phi, View, ViewAction};

// use sdl2::render::Renderer;
// use std::thread;
// use std::time::Duration;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().expect("sdl2 init failure");
    let video = sdl_context.video().expect("sdl video init failed");
    let mut timer = sdl_context.timer().expect("Failed to initilize sdl context timer");

    // Create window
    let window = video.window("ArcadeRS Shooter", 1024, 768 )
        .position_centered().opengl() // use OpenGL for faster rendering
        .build().expect("Window creation failed");

    // Create context
    let mut context = Phi {
        events: Events::new(sdl_context.event_pump().expect("failed to create new event context")),
                renderer: window.renderer()
                          .accelerated()
                          .build().unwrap(),
    };

    // Create default view
    // Note we're using the struct DefaultView in views/mod.rs
    let mut current_view: Box<View> = Box::new(::views::DefaultView);

    // Frame timing
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    // Iterate over event pump
    loop {
        // Frame timing (second instance)
        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        // If time elapsed since last frame is too small wait & try again
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        // Print FPS
        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }

        // Rendering logic
        context.events.pump();

        match current_view.render(&mut context, 0.01) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
        }
    }
}
