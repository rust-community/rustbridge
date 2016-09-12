#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;
use piston_window::*;
use std::thread;
use std::sync::mpsc;

// Generate a type that will produce a unique `widget::Id` for each widget.
widget_ids! {
    struct Ids {
        canvas,
        line,
        point_path,
        rectangle_fill,
        rectangle_outline,
        trapezoid,
        oval_fill,
        oval_outline,
        circle,
    }
}

//const WHITE:   types::Color = [1.0, 1.0, 1.0, 1.0];
const RED:     types::Color = [1.0, 0.0, 0.0, 1.0];
//const GREEN:   types::Color = [0.0, 1.0, 0.0, 1.0];
//const BLUE:    types::Color = [0.0, 0.0, 1.0, 1.0];
//const BLACK:   types::Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (paintsend, paintrecv) = mpsc::channel();
    let serverthread = thread::spawn(move || { serve_canvas (paintrecv) });
    let chn = paintsend.clone();
    let clientthread = thread::spawn(move || {
        elementarymondrian ([20.0, 20.0, 300.0, 250.0], chn)
    });
    let chn = paintsend.clone();
    let clientthread2 = thread::spawn(move || {
        elementarymondrian ([120.0, 220.0, 300.0, 250.0], chn)
    });
    serverthread.join().unwrap();
    clientthread.join().unwrap();
    clientthread2.join().unwrap();
}

fn elementarymondrian(r: types::Rectangle, chn: mpsc::Sender<(types::Rectangle, types::Color)>) {
    println! ( "putting: {:?}", (r, RED) );
    chn.send( (r, RED) ).unwrap();
}

fn serve_canvas (chn: mpsc::Receiver<(types::Rectangle, types::Color)>) {
    let mut canvas: Vec<(types::Rectangle, types::Color)> = Vec::new();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("Primitives Demo", [400, 720])
            .opengl(opengl).samples(4).exit_on_esc(true).build().unwrap();
    window.set_ups(60);

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();

    // A unique identifier for each widget.
    let ids = Ids::new(ui.widget_id_generator());

    // No text to draw, so we'll just create an empty text texture cache.
    let mut text_texture_cache = conrod::backend::piston_window::GlyphCache::new(&mut window, 0, 0);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    // Poll events from the window.
    while let Some(event) = window.next() {
        if let Ok( tobepainted ) = chn.try_recv() {
            canvas.push( tobepainted );
            println!("Received: {:?}", tobepainted );
        }

        // Convert the piston event to a conrod event.
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        // Update the widgets.
        event.update(|_| set_ui(ui.set_widgets(), &ids));

        // Draw the `Ui`.
        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }
}


fn set_ui(ref mut ui: conrod::UiCell, ids: &Ids) {
    use conrod::{Positionable, Widget};
    use conrod::widget::{Canvas, Circle, Line, Oval, PointPath, Polygon, Rectangle};
    use std::iter::once;

    // The background canvas upon which we'll place our widgets.
    Canvas::new().pad(80.0).set(ids.canvas, ui);

    Line::centred([-40.0, -40.0], [40.0, 40.0]).top_left_of(ids.canvas).set(ids.line, ui);

    let left = [-40.0, -40.0];
    let top = [0.0, 40.0];
    let right = [40.0, -40.0];
    let points = once(left).chain(once(top)).chain(once(right));
    PointPath::centred(points).down(80.0).set(ids.point_path, ui);

    Rectangle::fill([80.0, 80.0]).down(80.0).set(ids.rectangle_fill, ui);

    Rectangle::outline([80.0, 80.0]).down(80.0).set(ids.rectangle_outline, ui);

    let bl = [-40.0, -40.0];
    let tl = [-20.0, 40.0];
    let tr = [20.0, 40.0];
    let br = [40.0, -40.0];
    let points = once(bl).chain(once(tl)).chain(once(tr)).chain(once(br));
    Polygon::centred_fill(points).right_from(ids.line, 80.0).set(ids.trapezoid, ui);

    Oval::fill([40.0, 80.0]).down(80.0).align_middle_x().set(ids.oval_fill, ui);

    Oval::outline([80.0, 40.0]).down(100.0).align_middle_x().set(ids.oval_outline, ui);

    Circle::fill(40.0).down(100.0).align_middle_x().set(ids.circle, ui);
}
