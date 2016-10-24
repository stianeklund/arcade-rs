extern crate sdl2;
extern crate sdl2_image;

#[macro_use]
mod phi;
mod views;

fn main() {
    ::phi::spawn("ArcadeRS Shooter", |phi| {
        Box::new(::views::ShipView::new(phi))
    });
}
