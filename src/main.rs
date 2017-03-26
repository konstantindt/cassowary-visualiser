extern crate getopts;
extern crate gtk;
extern crate gdk;
extern crate cairo;
extern crate cassowary;

use std::env;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use getopts::Options;
use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition};
use visualiser::Visualiser;
use pen::PenStream;
use cassowary_calculations::{cal_demo1, cal_demo2, cal_demo3};

mod visualiser;
mod pen;
mod cassowary_calculations;

fn main() {
    // Skip the first argument. It tells us the path to this executable.
    let passed_arguments: Vec<String> = env::args().skip(1).collect();
    let opts = Options::new();
    let option_matches = match opts.parse(&passed_arguments) {
        Ok(o_match) => o_match,
        Err(_) => panic!("Failed to parse options."),
    };

    if option_matches.free.is_empty() {
        panic!("Expected 1 option to be passed as a command line argument which \
               selects a demo to run.");
    }

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

    let visu = Visualiser::new();
    window.add(visu.drawing_area());
    window.show_all();

    let shared_pen = Arc::new(Mutex::new(PenStream::new()));
    match option_matches.free[0].as_str() {
        "demo1" => {
            let mut pen = shared_pen.lock().unwrap();
            cal_demo1(visu.drawing_area().get_allocated_width() as f32,
                      visu.drawing_area().get_allocated_height() as f32,
                      pen.deref_mut());
        }
        "demo2" => {
            let mut pen = shared_pen.lock().unwrap();
            cal_demo2(30.0,
                      20.0,
                      130.0,
                      20.0,
                      30.0,
                      20.0,
                      20.0,
                      visu.drawing_area().get_allocated_width() as f32,
                      visu.drawing_area().get_allocated_height() as f32,
                      pen.deref_mut());
            // Key release mask (pressed)
            window.add_events(2048);
            visu.set_key_pressed_event(shared_pen.clone());
        }
        "demo3" => {
            let mut pen = shared_pen.lock().unwrap();
            cal_demo3(30.0,
                      20.0,
                      150.0,
                      20.0,
                      300.0,
                      20.0,
                      30.0,
                      20.0,
                      20.0,
                      visu.drawing_area().get_allocated_width() as f32,
                      visu.drawing_area().get_allocated_height() as f32,
                      pen.deref_mut());
            visu.set_size_change_event(shared_pen.clone());
        }
        "demo4" => {
            let mut pen = shared_pen.lock().unwrap();
            cal_demo3(30.0,
                      20.0,
                      150.0,
                      20.0,
                      300.0,
                      20.0,
                      30.0,
                      20.0,
                      20.0,
                      visu.drawing_area().get_allocated_width() as f32,
                      visu.drawing_area().get_allocated_height() as f32,
                      pen.deref_mut());
            visu.set_mouse_drag_event(shared_pen.clone());
        }
        _ => panic!("Demo selection not recognised."),
    }

    visu.set_draw_event(shared_pen.clone());
    visu.set_mouse_move_event(shared_pen.clone());

    window.show_all();
    gtk::main();
}
