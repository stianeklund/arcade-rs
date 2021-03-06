// phi/events.rs
// Note: we're creating a macro for later usage.
macro_rules! struct_events {
    (
    keyboard: { $( $k_alias:ident : $k_sdl:ident ),* },

    // Match against a pattern
    else: { $( $e_alias:ident : $e_sdl:pat ),* }
    )
    => {
        use sdl2::EventPump;

        pub struct ImmediateEvents {
        // For every keyboard event, we have a Option<bool>
        // Some(true) key was pressed
        // Some(false) key was released (from being pressed)
        // None => Nothing has happened
        resize: Option<(u32, u32)>,
        $( pub $k_alias: Option<bool> , )*
        $( pub $e_alias: bool ),*
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    resize: None,
                    // When reinitalized nothing has happened yet
                    $( $k_alias: None , )*
                    $( $e_alias: false ),*
                }
            }
        }

        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,

            // true = key pressed (see ImmediateEvents)
            $(pub $k_alias: bool ),*
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),
                    // Initialize keys as not pressed; the opposite = issues
                    $( $k_alias: false ),*
                }
            }
            pub fn pump(&mut self, renderer: &mut ::sdl2::render::Renderer) {
                // If SDL context is dropped, poll_iter() will not yield output
                self.now = ImmediateEvents::new();

                // Iter over eventpump & poll for events, Esc being one of them
                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::event::WindowEventId::Resized;
                    use sdl2::keyboard::Keycode::*;

                    // For match statement below
                    // $( ... ),* containing $k_sdl and $k_alias means: for every element ($k_alias : $k_sdl) pair,
                    // check whether the keycode is Some($k_sdl). If it is, then set the $k_alias fields to true."

                    match event {
                        Window { win_event_id: Resized, .. } => {
                            self.now.resize = Some(renderer.output_size().unwrap());
                        },
                        KeyDown { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // Prevent multiple presses when holding a key down.
                                    // Was previously not pressed?
                                    if !self.$k_alias {
                                        self.now.$k_alias = Some(true);
                                    }

                                    self.$k_alias = true;
                                }
                             ),* // add comma after every option
                             _ => {}
                        },

                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // key released
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false;
                                }
                             ),*
                            _ => {}
                        },

                        $(
                            $e_sdl => {
                                self.now.$e_alias = true;
                            }
                         )*,

                        _ => {}
                    }
                }
            }
        }
    }
}
