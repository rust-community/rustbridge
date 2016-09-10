extern crate graphics;
extern crate find_folder;
extern crate piston_window;

use piston_window::*;

const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const LIGHTRED:     [f32; 4] = [1.0, 0.5, 0.5, 1.0];
const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("Primitives Demo", [400, 720])
            .opengl(opengl).samples(4).exit_on_esc(true).build().unwrap();
    window.set_ups(60);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            let redrect = graphics::Rectangle::new(WHITE).border(graphics::rectangle::Border{color :BLACK, radius :2.0});
            redrect.color(LIGHTRED).draw([20.0, 20.0, 50.0, 60.0], &c.draw_state, c.transform, g);
        });
    }

}
