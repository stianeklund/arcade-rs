// phi/mod.rs

#[macro_use]
mod events;

use sdl2::render::Renderer;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
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

    // Create window
    let window = video.window("ArcadeRS Shooter", 1024, 768 )
        .position_centered().opengl() // use OpenGL for faster rendering
        .build().expect("Window creation failed");

    // Create context
    let mut context = Phi {
        events: Events::new(sdl_context.event_pump().expect("failed to create new event context")),
        renderer: window.renderer()
            .accelerated()
            .build().expect("window rendering failed"),
    };

    // Create default view
    // Note we're using the struct DefaultView in views/mod.rs
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

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) => current_view = new_view,

        }
    }
}
