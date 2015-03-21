extern crate simple;
use simple::{Window, Rect};

extern crate rand;
use rand::Rng;


static WIDTH:i32 = 800;
static HEIGHT:i32 = 600;


fn main() {
    let mut app = simple::Window::new("hello world", WIDTH, HEIGHT);
    let mut rng = rand::thread_rng();

    let rect_max_size = 300;

    while app.next_frame() {
        app.set_color(
            rng.gen_range(0, 255),
            rng.gen_range(0, 255),
            rng.gen_range(0, 255),
            rng.gen_range(0, 255),
        );
        app.draw_rect(simple::Rect{
            x: rng.gen_range(0, WIDTH),
            y: rng.gen_range(0, HEIGHT),
            w: rng.gen_range(-rect_max_size, rect_max_size),
            h: rng.gen_range(-rect_max_size, rect_max_size),
        });
    }
}
