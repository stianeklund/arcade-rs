// views/mod.rs

use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use phi::gfx::{CopySprite, Sprite};
use sdl2::pixels::Color;
use sdl2::render::Renderer;

// Pixels traversed very second when the ship is moving
const PLAYER_SPEED: f64 = 180.0;

const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;

const DEBUG: bool = false;

#[derive(Clone)]
struct Background {
    pos: f64,
    // The amount of pixels moved to the left every second
    vel: f64,
    sprite: Sprite,
}


impl Background {
    fn render(&mut self, renderer: &mut Renderer, elapsed: f64) {
        let size = self.sprite.size();
        self.pos += self.vel * elapsed;

        if self.pos > size.0 {
            self.pos -= size.0;
        }

        // Determine the scale ratio of the window to the sprite
        let (win_w, win_h) = renderer.output_size().unwrap();
        let scale = win_h  as f64 / size.1;

        // Render as many copies of the background as needed to fill the screen.
        let mut physical_left = -self.pos * scale;

        while physical_left < win_w as f64 {
                renderer.copy_sprite(&self.sprite, Rectangle {
                    x: physical_left,
                    y: 0.0,
                    w: size.0 * scale,
                    h: win_h as f64,
                });
                physical_left += size.0 * scale;
        }
    }
}

#[derive(Clone, Copy)]
enum ShipFrame {
    UpNorm   = 0,
    UpFast   = 1,
    UpSlow   = 2,
    MidNorm  = 3,
    MidFast  = 4,
    MidSlow  = 5,
    DownNorm = 6,
    DownFast = 7,
    DownSlow = 8
}

struct Ship {
    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: ShipFrame,
}

pub struct ShipView {
    player: Ship,

    bg_back: Background,
    bg_middle: Background,
    bg_front: Background,

}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {

        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png")
            .expect("Failed to load main asset");
        // Allocate sprite data (we already know Vec capacity).
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
                    w: SHIP_W,
                    h: SHIP_H,
                },
                sprites: sprites,
                current: ShipFrame::MidNorm,
            },
            bg_back: Background {
                pos: 0.0,
                vel: 20.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starBG.png")
                    .expect("failed to load background asset"),
            },
            // TODO Investigate stretched background middle asset
            bg_middle: Background {
                pos: 0.0,
                vel: 40.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starMG.png")
                    .expect("failed to load middle asset"),
            },
            bg_front: Background {
                pos: 0.0,
                vel: 80.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starFG.png")
                    .expect("failed to load foreground asset"),
            },


        }
    }
}


// View definitions

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // Player movement
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


        // Restrict players movable region
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
        // Select the appropriate sprite of the ship to show.
        self.player.current =
            if dx == 0.0 && dy < 0.0       { ShipFrame::UpNorm }
            else if dx > 0.0 && dy < 0.0   { ShipFrame::UpFast }
            else if dx < 0.0 && dy < 0.0   { ShipFrame::UpSlow }
            else if dx == 0.0 && dy == 0.0 { ShipFrame::MidNorm }
            else if dx > 0.0 && dy == 0.0  { ShipFrame::MidFast }
            else if dx < 0.0 && dy == 0.0  { ShipFrame::MidSlow }
            else if dx == 0.0 && dy > 0.0  { ShipFrame::DownNorm }
            else if dx > 0.0 && dy > 0.0   { ShipFrame::DownFast }
            else if dx < 0.0 && dy > 0.0   { ShipFrame::DownSlow }
            else { unreachable!() };

        // Clear screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render the backgrounds
        self.bg_back.render(&mut phi.renderer, elapsed);
        self.bg_middle.render(&mut phi.renderer, elapsed);

        // Render bounding box (for debugging)
        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());
        }

        // Render the ship
        phi.renderer.copy_sprite(
            &self.player.sprites[self.player.current as usize],
            self.player.rect);

        // Render the foreground
        self.bg_front.render(&mut phi.renderer, elapsed);

        ViewAction::None
    }
}
