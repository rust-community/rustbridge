// Combines the multiprint and graphout https://github.com/broesamle/RustWorkshop/blob/master/minimals/multiprint.md examples into a minimal multithreaded 'paint server' scheme.

extern crate graphics;
extern crate piston_window;
extern crate rand;

use piston_window::*;

use graphics::types::{Color};

use rand::Rng;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED:     [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (paintsend, paintrecv) = mpsc::channel();

    let serverthread = thread::spawn(move || {
        let mut pleaseinit = true;

        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Construct the window.
        let mut window: PistonWindow =
            WindowSettings::new("Primitives Demo", [400, 720])
                .opengl(opengl).samples(4).exit_on_esc(true).build().unwrap();
        window.set_ups(60);

        while let Some(e) = window.next() {
                window.draw_2d(&e, |c, gl| {
                let redrect = graphics::Rectangle::new(WHITE).border(graphics::rectangle::Border{color :BLACK, radius :2.0});

                if pleaseinit {
                    clear(WHITE, gl);
                    pleaseinit = false;
                }
                if let Ok( (rct, col) ) = paintrecv.recv() {
                    println!("Received: {:?}", (rct, col) );
                    redrect.color(col).draw(rct, &c.draw_state, c.transform, gl);
                }
            });
        }
    });
    let chn = paintsend.clone();
    let clientthread = thread::spawn(move ||
        delegatemondrian([20.0, 20.0, 300.0, 250.0], chn)
    );
    println!("Started master painter.");

    let _ = clientthread.join();
    let _ = serverthread.join();
}

fn delegatemondrian(r: types::Rectangle, chn: mpsc::Sender<(types::Rectangle, Color)>) {
    let mut rng = rand::thread_rng();   //init a random number generator

    let (x, y, w, h) = (r[0], r[1], r[2], r[3]);
    let splitpos = rng.gen_range(0.0, w);
    let leftsection = [x, y, splitpos, h];
    let rightsection = [x+splitpos, y, w-splitpos, h];

    let chnleft = chn.clone();
    let leftpainter = thread::spawn(move ||
        elementarymondrian(leftsection, chnleft)
    );
    thread::sleep(Duration::from_millis(500));
    println!("Delegated left.");
    let chnright = chn.clone();
    let rightpainter = thread::spawn(move ||
        elementarymondrian(rightsection, chnright)
    );
    println!("Delegated right.");
    let _ = leftpainter.join();
    let _ = rightpainter.join();
}

fn elementarymondrian(r: types::Rectangle, chn: mpsc::Sender<(types::Rectangle, Color)>) {
    println! ( "putting: {:?}", (r, RED) );
    chn.send( (r, RED) ).unwrap();
}
