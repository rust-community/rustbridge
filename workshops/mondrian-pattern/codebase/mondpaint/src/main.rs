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
//use graphics::draw_state::DrawState;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED:     [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const SCALE: f64 = 20.0;


fn main() {
    // prepare queue for inter-process communication
    let mut threads = Vec::new();
    let (paintsend, paintrecv) = mpsc::channel();

    let server = thread::spawn(move || {
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
    for num in 0..10 {
        let chn = paintsend.clone();
        let handle = thread::spawn(move || {
            let mut i = 0.0;
            loop {
                println!("putting: {:?}", (i, num));
                let (x, y, w, h) = (i*SCALE, (num as f64)*SCALE, 15.0, 15.0);
                chn.send( ([x, y, w, h], RED) ).unwrap();
                i += 1.0;
                thread::sleep(Duration::from_millis(500*(num+1)));
            }
        });
        threads.push(handle);
        println!("Started thread number {:?}.", num);
    }
    for num in (0..10).rev() {
        let thr = threads.remove(num);
        let joinresult = thr.join();
        println!("Joined thread number {:?}, {:?}.", num, joinresult);
    }
    let _ = server.join();
}
