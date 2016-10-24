// views/mod.rs

use std::path::Path;
use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use phi::gfx::{Sprite};
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureQuery};
use sdl2_image::LoadTexture;

// Pixels traversed every second when the ship is moving
const PLAYER_SPEED: f64 = 180.0;
const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;


// View definitions

struct Ship {
    rect: Rectangle,
    sprite: Sprite,
}

pub struct ShipView {
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        let sprite = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        // NOTE size method takes &self and returns (f64, f64), of which we pass src.w, src.h)
        let (w, h) = sprite.size();


        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: w,
                    h: h,
                },
                sprite: sprite, // load sprite instead
                // TODO Remove:
                // tex: phi.renderer.load_texture(Path::new("assets/spaceship.png")).expect("Failed to load asset")
            }
        }
    }
}


impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }


        // Move the ship
        let diagonal =
            (phi.events.key_up ^ phi.events.key_down) &&
            (phi.events.key_left ^ phi.events.key_right);

        let moved =
            if diagonal { 1.0 / 2.0f64.sqrt() }
            else { 1.0 } * PLAYER_SPEED * elapsed;

        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };
        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true,true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;


        // Restrict width
        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };
        // If the player cannot fit within the frame, abort
        self.player.rect = self.player.rect
            .move_inside(movable_region)
            .expect("Player cannot fit in screen");

        // Clear screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render bounding box (for debugging)
        phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
        phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());

        // Render the ship
        // TODO fix no method render found for type phi::gfx::Sprite in current scope;
        self.player.sprite.render(&mut phi.renderer, self.player.rect);
        // phi.renderer.copy(&mut self.player.tex,
        // Rectangle {
            // x: SHIP_W * 0.0,
            // y: SHIP_H * 1.0,
            // w: self.player.rect.w,
            // h: self.player.rect.h,
        // }.to_sdl(),


        self.player.rect.to_sdl()
            .unwrap();

        ViewAction::None
    }
}
