// phi/data.rs
use sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {

    /// API changes in SDL2 stack 0.24 & up. Comments below apply to v.0.24 and up only.
    /// sdl2 renderer drawing methods return Result<(), String) which need to be unwrapped.
    /// Rect::new (0.24 & up) no longer returns a Result type, but the value of Rect itself.

    /// to_sdl create's an SDL compatible Rect equivilent to self.
    /// Workaround for SDL2 0.24 stack is that SdlRect::new is wrapped within Some() as a workaround as
    /// SdlRect::new used to return a Result type.

    pub fn to_sdl(self) -> Option<SdlRect> {
        assert!(self.w >= 0.0 && self.h >= 0.0);
        Some(SdlRect::new(
                self.x as i32, self.y as i32, self.w as u32, self.h as u32
                ))
    }

    pub fn contains(&self, rect: Rectangle) -> bool {
        let xmin = rect.x;
        let xmax = xmin + rect.w;
        let ymin = rect.y;
        let ymax = ymin + rect.h;

        xmin >= self.x && xmin <= self.x + self.w &&
        xmax >= self.x && xmax <= self.x + self.w &&
        ymin >= self.y && ymin <= self.y + self.h &&
        ymax >= self.y && ymax <= self.y + self.h
    }

    pub fn overlaps(&self, other: Rectangle) -> bool {
        self.x < other.x + other.w &&
        self.x + self.w > other.x &&
        self.y < other.y + other.h &&
        self.y + self.h > other.y
    }

    // This defines our allowed area of movement within the parent rectangle
    pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
        if self.w > parent.w || self.h > parent.h {
            return None;
        }

        Some(Rectangle {
            w: self.w,
            h: self.h,
            x: if self.x < parent.x { parent.x }
                else if self.x + self.w >= parent.x + parent.w { parent.x + parent.w - self.w }
                else { self.x },
            y: if self.y < parent.y { parent.y }
                else if self.y + self.h >= parent.y + parent.h { parent.y + parent.h - self.h }
                else { self.y },
        })
    }
    }
