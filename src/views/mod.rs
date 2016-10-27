// views/mod.rs

use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use phi::gfx::{CopySprite, Sprite};
use sdl2::pixels::Color;

// Pixels traversed very second when the ship is moving
const PLAYER_SPEED: f64 = 180.0;
const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;


// View definitions

struct Ship {
    rect: Rectangle,
    sprite: Vec<Sprite>,
    current: ShipFrame,
}

pub struct ShipView {
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        // let sprite = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        // NOTE size method takes &self and returns (f64, f64), of which we pass src.w, src.h)
        // let (w, h) = sprite.size();

        // TODO Implement spritesheet
        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        // Allocate sprite data (we already Vec capacity).
        let mut sprites = Vec::with_capacity(9);
        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                    w: SHIP_W,
                    h: SHIP_H,
                    x: SHIP_W * x as f64,
                    y: SHIP_H * y as f64,
                }).unwrap());
            }
        }

        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: w,
                    h: h,
                },
                sprite: sprite,
                current: ShipFrame::MidNorm,
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
        // phi.renderer.copy(&mut self.player.tex,
        self.player.sprite.render(&mut phi.renderer, self.player.rect);

        ViewAction::None
    }
}
