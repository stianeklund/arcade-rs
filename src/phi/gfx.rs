// src/phi/gfx.rs

use phi::data::Rectangle;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;

/// RefCell<T> provides interior mutability. Enforces RWLock pattern at runtime
/// This is much like a single-threaded mutex behavior wise.
/// The internal reference count gets modified & returns smart pointers, which can be
/// dereferenced immutably & mutably. Refcount is restored when smart pointers go out of scope.
/// TODO: Read up on RefCell to get a better understanding of this.

/// It's likely needed for sprite usage as parts of the asset is cloned as different areas of the
/// image is used to generate different sprites?

pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}

// Implementing the Clone trait for Sprite manually (for learning purposes).
// Quick way to do this is to tell the compiler to derive the Clone trait:
// #[derive(Clone)]

impl Clone for Sprite {
    fn clone(&self) -> Sprite {
        Sprite {
            tex: self.tex.clone(),
            src: self.src.clone(),
        }
    }
}


impl Sprite {
    // Creates a new sprite by wrapping a texture
    pub fn new(texture: Texture) -> Sprite {
        let tex_query = texture.query();

        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rectangle {
                w: tex_query.width as f64,
                h: tex_query.width as f64,
                x: 0.0,
                y: 0.0,
            }
        }
    }

    // Creates a new sprite from the asset image. Note: the Option return type.
    pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }

    /// Returns a new Sprite representing a sub-region of the current one (i.e a part of the image).
    /// The provided 'rect' is relative to the currently held region.
    /// Again, note the return Type.

    pub fn region(&self, rect: Rectangle) -> Option<Sprite> {
        let new_src = Rectangle {
            x: rect.x + self.src.x,
            y: rect.y + self.src.y,
            ..rect // Note: .. (Implicitly saying, skip the rest)
        };

        // Verify that the requested region is valid and inside of the current one
        if self.src.contains(new_src) {
            Some(Sprite {
                tex: self.tex.clone(),
                src: new_src,
            })
        } else {
            None
        }
    }

    // Returns the dimensions of our region
    pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }

    /// TODO: Look into renderer method & borrow_mut method
    /// borrow.mut() is probably similar to refcell. Are we temporarily borrowing mutability of
    /// self? Wat?

    // TODO
    // Mismatched types, expected () found Result<(), std::string::String>
    pub fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        renderer.copy(&mut self.tex.borrow_mut(), self.src.to_sdl(), dest.to_sdl())
    }
}
pub trait CopySprite {
    fn copy_sprite(&mut self, sprite: &Sprite, dest: Rectangle);
}

impl<'window> CopySprite for Renderer<'window> {
    fn copy_sprite(&mut self, sprite: &Sprite, dest: Rectangle) {
        sprite.render(self, dest);
    }
}

