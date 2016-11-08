extern crate gtk;
extern crate cairo;

use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition, DrawingArea};
use draw_events::set_draw_callback;

mod draw_events;
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

    let drawing_area = &DrawingArea::new();
    set_draw_callback(drawing_area);
    window.add(drawing_area);

    window.show_all();
    gtk::main();
}
