extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::Renderer;
use std::thread;
use std::time::Duration;

#[macro_use]
mod events;

// can be passed easily between functions.
// note lifetime specifier
pub struct Phi<'a> {
    pub events: Events,
    pub renderer: Renderer<'a>,
}
// Specify action before passed to View to be rendered
pub enum ViewAction {
    None,
    Quit,
}

pub trait View {
    // TODO View is called every frame. Responsible for redering current view.
    // Elapsed time is in seconds
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    },
    else: {
        quit: Quit { .. }
    }
}

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().expect("Init failed");
    let video = sdl_context.video().expect("Init of video failed");

    // Create window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl() // use OpenGL for faster rendering
        .build().expect("Window creation failed");

    // Create Renderer
    let mut renderer = window.renderer()
        .accelerated()
        .build().expect("Renderer failed");

    // Prepare events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    // Iterate over EventPump & quit if escape is pressed
    loop {
        events.pump();
        if events.now.quit || events.now.key_escape == Some(true) {
        break;
        }

    // Render backdrop
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();
    }


    thread::sleep(Duration::from_millis(3000));
}
