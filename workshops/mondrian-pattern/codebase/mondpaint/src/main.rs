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

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    // prepare queue for inter-process communication
    let mut threads = Vec::new();
    let queue: Vec<(f64, f64)> = Vec::new();
    let printqueue_mutex_arc = Arc::new(Mutex::new(queue));
    let serverqueue = printqueue_mutex_arc.clone();

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
                gl.draw(r.viewport(), |_, gl| {
                    if pleaseinit {
                        clear(WHITE, gl);
                        pleaseinit = false;
                    }
                    if let Ok(mut guard) = serverqueue.try_lock() {
                        if let Some(_) = (*guard).pop() {
                        }
                    }
                });
            }
        }
    });
    for num in 0..10 {
        let clientqueue = printqueue_mutex_arc.clone();
        let handle = thread::spawn(move || {
            let mut i = 0.0;
            loop {
                if let Ok(mut guard) = clientqueue.try_lock() {
                    println!("putting: {:?}", (i, num));
                    (*guard).push((i, num as f64));
                    i += 1.0;
                }
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
