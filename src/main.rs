extern crate sdl2;
extern crate sdl2_image;

#[macro_use]
mod phi;
mod views;
// http://jadpole.github.io/arcaders/arcaders-1-6
fn main() {
    ::phi::spawn("ArcadeRS Shooter", |phi| {
        Box::new(::views::ShipView::new(phi))
    });
}
