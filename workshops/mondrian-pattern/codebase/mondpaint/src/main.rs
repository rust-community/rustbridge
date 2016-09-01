// Combines the multiprint and graphout https://github.com/broesamle/RustWorkshop/blob/master/minimals/multiprint.md examples into a minimal multithreaded 'paint server' scheme.

extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::Events;
use piston::input::RenderEvent;
use graphics::clear;
use graphics::types::{Rectangle, Color};

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
        // prepare graphics output + window management
        let mut window: Window =
            WindowSettings::new("Hello World!", [512; 2])
                .build().unwrap();
        let opengl = OpenGL::V3_2;
        let mut gl = GlGraphics::new(opengl);
        let mut events = window.events();

        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                gl.draw(r.viewport(), |c, gl| {
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
        }
    });
    let chn = paintsend.clone();
    let clientthread = thread::spawn(move ||
        elementarymondrian([20.0, 20.0, 300.0, 250.0], chn)
    );
    println!("Started master painter.");

    let _ = clientthread.join();
    let _ = serverthread.join();
}

fn delegatemondrian(r: Rectangle, chn: mpsc::Sender<(Rectangle, Color)>) {
    let (x, y, w, h) = (r[0], r[1], r[2], r[3]);
    let leftsection: Rectangle = [x, y, w/2.0, h];
    let rightsection: Rectangle = [x+w/2.0, y, w/2.0, h];

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

fn elementarymondrian(r: Rectangle, chn: mpsc::Sender<(Rectangle, Color)>) {
    println! ( "putting: {:?}", (r, RED) );
    chn.send( (r, RED) ).unwrap();
}
