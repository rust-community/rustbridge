extern crate graphics;
extern crate piston_window;
extern crate rand;

use piston_window::*;

//use rand::Rng;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

type SendChannel = mpsc::Sender<(types::Rectangle, types::Color)>;

const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED:     [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN:   [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE:    [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const YELLOW:  [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (paintsend, paintrecv) = mpsc::channel();

    let serverthread = thread::spawn(move || {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Construct the window.
        let mut window: PistonWindow =
            WindowSettings::new("RustBridge / Mondrian Pattern Generator", [400, 720])
                .opengl(opengl).samples(4).exit_on_esc(true).build().unwrap();
        window.set_ups(60);

        let mut canvas: Vec<(types::Rectangle, types::Color)> = Vec::new();
        while let Some(e) = window.next() {
            if let Ok( tobepainted ) = paintrecv.try_recv() {
                canvas.push( tobepainted );
                println!("Painter received: {:?}", tobepainted );
            }
            window.draw_2d(&e, |c, gl| {
                clear(WHITE, gl);
                let redrect = graphics::Rectangle::new(WHITE).border(graphics::rectangle::Border{color :BLACK, radius :2.0});
                for item in (&canvas).iter() {
                    let (rct, col) = *item;
                    redrect.color(col).draw(rct, &c.draw_state, c.transform, gl);
                }
            });
        }
    });
    let chn = paintsend.clone();
    let painterthread = thread::spawn(move ||
        paint_rectangle(20.0, 20.0, 300.0, 250.0, chn)
    );
    painterthread.join().unwrap();
    serverthread.join().unwrap();
}


fn vsplit_and_paint(x :f64, y :f64, width :f64, height :f64, chn: SendChannel) {
    // let mut rng = rand::thread_rng();   //init a random number generator
    println!("vsplit_and_paint: {:}, {:}, {:}, {:}", x, y, width, height);

    let splitpos = width / 2.0;
    let chnleft = chn.clone();
    let leftpainterthread = thread::spawn(move ||
        paint_rectangle(x, y, splitpos, height, chnleft)
    );
    thread::sleep(Duration::from_millis(500));
    let chnright = chn.clone();
    let rightpainterthread = thread::spawn(move ||
        paint_rectangle(x+splitpos, y, width-splitpos, height, chnright)
    );
    leftpainterthread.join().unwrap();
    rightpainterthread.join().unwrap();
}

fn paint_rectangle(x :f64, y :f64, width :f64, height :f64, chn: SendChannel)
{
    println!("paint_rectangle: {:}, {:}, {:}, {:}", x, y, width, height);
    chn.send( ([x, y, width, height], RED) ).unwrap();
}
