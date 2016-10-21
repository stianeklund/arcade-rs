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

    // Create's an SDL compatible Rect equivilent to self.
    // Panics if coordinate overflows an i32
    pub fn to_sdl(self) -> Option<SdlRect> {
        // Negative width and height gets rejected
        assert!(self.w >= 0.0 && self.h >= 0.0);

        // SdlRect::new : `(i32, i32, u32, u32) -> Result<Option<SdlRect>>`
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
            .expect("Failed to create new SdlRect")
    }

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
