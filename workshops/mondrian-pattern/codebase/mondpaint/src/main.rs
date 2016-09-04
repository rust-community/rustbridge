extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::Events;
use piston::input::RenderEvent;
use graphics::clear;
use graphics::types::{Rectangle, Color};

use rand::Rng;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED:     [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN:   [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE:    [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const YELLOW:  [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (paintsend, paintrecv) = mpsc::channel();

    let gfxserverthread = thread::spawn(move || {
        let mut pleaseinit = true;
        // prepare graphics output + window management
        let mut window: Window =
            WindowSettings::new("RustBridge / Mondrian Pattern Generator", [512; 2])
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
                    if let Ok( (rct, col) ) = paintrecv.try_recv() {
                        println!("GFX-OUTPUT: rect {:?}", (rct, col) );
                        redrect.color(col).draw(rct, &c.draw_state, c.transform, gl);
                    }
                });
            }
        }
    });
    let chn = paintsend.clone();
    let patternpainterthread = thread::spawn(move ||
        hsplit_and_paint(20.0, 20.0, 300.0, 250.0, chn)
    );

    patternpainterthread.join().unwrap();
    gfxserverthread.join().unwrap();
}

fn paint_rectangle(x :f64, y :f64, width :f64, height :f64, c: Color, chn: mpsc::Sender<(Rectangle, Color)>)
{
    println! ( "paint_rectangle: {:}, {:}, {:}, {:}", x, y, width, height);
    chn.send( ([x, y, width, height], c) ).unwrap();
}

fn vsplit_and_paint(x :f64, y :f64, width :f64, height :f64, chn: mpsc::Sender<(Rectangle, Color)>) {
    let mut rng = rand::thread_rng();   //init a random number generator

    let splitpos = rng.gen_range(0.0, width);

    let chnleft = chn.clone();
    let leftpainterthread = thread::spawn(move ||
        paint_rectangle(x, y, splitpos, height, RED, chnleft)
    );
    thread::sleep(Duration::from_millis(500));
    let chnright = chn.clone();
    let rightpainterthread = thread::spawn(move ||
        paint_rectangle(x+splitpos, y, width-splitpos, height, BLUE, chnright)
    );
    let _ = leftpainterthread.join();
    let _ = rightpainterthread.join();
}

fn hsplit_and_paint(x :f64, y :f64, width :f64, height :f64, chn: mpsc::Sender<(Rectangle, Color)>) {
    let mut rng = rand::thread_rng();   //init a random number generator

    let splitpos = rng.gen_range(0.0, height);

    let chnleft = chn.clone();
    let upperpainterthread = thread::spawn(move ||
        vsplit_and_paint(x, y, width, splitpos, chnleft)
    );
    thread::sleep(Duration::from_millis(500));
    let chnright = chn.clone();
    let lowerpainterthread = thread::spawn(move ||
        vsplit_and_paint(x, y+splitpos, width, height-splitpos, chnright)
    );
    let _ = upperpainterthread.join();
    let _ = lowerpainterthread.join();
}
