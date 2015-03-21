extern crate simple;
use simple::{Window, Rect};

fn main() {
    let mut app = simple::Window::new("hello world", 1920, 1080);

    app.set_color(255, 0, 255, 255);
    app.draw_rect(simple::Rect{
        x: 100,
        y: 100,
        w: 300,
        h: 200,
    });
    
    while app.next_frame() {}
}
