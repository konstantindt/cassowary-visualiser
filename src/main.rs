extern crate getopts;
extern crate gtk;
extern crate gdk;
extern crate cairo;
extern crate cassowary;

use std::env;
use std::sync::{Arc, Mutex};
use getopts::Options;
use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition};
use visualiser::Visualiser;
use cassowary::math::variables::{new_var, new_const};
use cassowary::math::relationships::Relationship;
use cassowary::math::expressions::Expression;
use cassowary::objective::functions::Function;
use cassowary::objective::problems::ProblemType;
use cassowary::objective::constraints::{new_reg_con, SystemOfConstraints};
use pen::PenStream;

mod visualiser;
mod pen;

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
            let x_loc = 30.0;
            let y_loc = 20.0;
            let side_margin = 30.0;
            let top_margin = 20.0;
            let da_width = visu.drawing_area().get_allocated_width() as f32;
            let da_height = visu.drawing_area().get_allocated_height() as f32;
            let exp1 = Expression::new(vec![new_var("P", 1.0)],
                                       Relationship::EQ,
                                       vec![new_var("width", 1.0), new_var("height", 1.0)]);
            let exp2 = Expression::new(vec![new_var("width", 1.0)],
                                       Relationship::LEQ,
                                       vec![new_const("con1", da_width - 2.0 * side_margin)]);
            let exp3 = Expression::new(vec![new_var("height", 1.0)],
                                       Relationship::LEQ,
                                       vec![new_const("con2", da_height - 2.0 * top_margin)]);
            let mut objective_func = Function::new(exp1, ProblemType::MAX);
            let c1 = new_reg_con(exp2);
            let c2 = new_reg_con(exp3);
            let subject_to = SystemOfConstraints::new(vec![c1, c2]);
            let solution = cassowary::optimise(&mut objective_func, &subject_to);
            let width = solution.iter()
                .find(|&entry| entry.0 == "width")
                .unwrap()
                .1;
            let height = solution.iter()
                .find(|&entry| entry.0 == "height")
                .unwrap()
                .1;
            println!("width = {}\nheight = {}", width, height);
            pen.add_rec_to_draw(x_loc as f64, y_loc as f64, width as f64, height as f64);
        }
        _ => panic!("Demo selection not recognised."),
    }

    visu.set_draw_event(shared_pen.clone());
    visu.set_mouse_move_event(shared_pen.clone());

    window.show_all();
    gtk::main();
}
