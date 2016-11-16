extern crate gtk;
extern crate cairo;

use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition};
use visualiser::Visualiser;
use pen::PenStream;

mod visualiser;
mod pen;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialise GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Positions Visualiser");
    window.set_default_size(400, 300);
    window.set_position(WindowPosition::Center);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let mut pen = PenStream::new();
    pen.add_rec_to_draw(30.0, 30.0, 30.0, 40.0);
    pen.add_rec_to_draw(90.0, 90.0, 30.0, 40.0);
    pen.add_rec_to_draw(150.0, 150.0, 30.0, 40.0);
    pen.add_rec_to_draw(210.0, 210.0, 30.0, 40.0);

    let visu = Visualiser::new();
    visu.set_draw_callback(pen);
    window.add(visu.get_drawing_area());

    window.show_all();
    gtk::main();
}
