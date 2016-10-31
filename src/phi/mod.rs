// phi/mod.rs

#[macro_use]
mod events;
pub mod data;
pub mod gfx;

use sdl2::render::Renderer;

// Enable for console FPS output
const DEBUG: bool = false;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
    },
    else: {
        quit: Quit { .. }
    }
}

pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

/// Init logic for Phi to new method called new.
/// Note Phi::new is not public (We do not want users of the library to create their own context.
/// If they did, sdl2_image lib could be freed at any time, which would remove all guarantees of the
/// image loading.

impl <'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
        Phi {
            events: events,
            renderer: renderer,
        }
    }
    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }
}
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    // Called on every frame. Handles both logic and rendering of current view.
    // Elapsed time is expressed in seconds
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}
pub fn spawn<F>(title: &str, init: F)
    where F: Fn(&mut Phi) -> Box<View> {

    // Initialize SDL2
    let sdl_context = ::sdl2::init().expect("sdl2 init failure");
    let video = sdl_context.video().expect("sdl video init failed");
    let mut timer = sdl_context.timer().expect("sdl context timer failed");
    let _image_context = ::sdl2_image::init(::sdl2_image::INIT_PNG).unwrap();

    // Create window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered()// .opengl()
        .build().expect("Window creation failed");

    // Create new instance of Phi
    let mut context = Phi::new(
        Events::new(sdl_context.event_pump().unwrap()),
        window.renderer()
        .accelerated()
        .build().unwrap());

    // Create view
    // let mut current_view: Box<View> = Box::new(::views::DefaultView);
    let mut current_view = init(&mut context);

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
            before = now;
            fps += 1;
            if now - last_second > 1_000 {
                if DEBUG {
                println!("FPS: {}", fps);
                }
                last_second = now;
                fps = 0;
            }

        // Rendering logic
        context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) => current_view = new_view,

        }
    }
}
