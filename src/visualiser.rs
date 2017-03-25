use std::ops::DerefMut;
use std::sync::{Arc, Mutex, MutexGuard};
use gtk::prelude::*;
use gtk::{DrawingArea, Allocation};
use gdk::{EventButton, EventKey, EventMotion};
use cairo::Context;
use pen::PenStream;
use cassowary_calculations::{demo2_key_release, demo3_size_change};

pub struct Visualiser {
    drawing_area: DrawingArea,
    shared_command_stack: Arc<Mutex<Vec<DrawCommand>>>,
    shared_marked_rec: Arc<Mutex<Option<usize>>>,
    shared_drag_start_pos: Arc<Mutex<Option<(i32, i32)>>>,
}

impl Visualiser {
    pub fn new() -> Visualiser {
        Visualiser {
            drawing_area: DrawingArea::new(),
            shared_command_stack: Arc::new(Mutex::new(vec![DrawCommand::DrawAll,
                                                           DrawCommand::DrawAll])),
            shared_marked_rec: Arc::new(Mutex::new(None)),
            shared_drag_start_pos: Arc::new(Mutex::new(None)),
        }
    }

    pub fn drawing_area(&self) -> &DrawingArea {
        &self.drawing_area
    }

    pub fn set_draw_event(&self, draw_with: Arc<Mutex<PenStream>>) {
        let cs_handle = self.shared_command_stack.clone();
        self.drawing_area.connect_draw(move |_: &DrawingArea, cr: &Context| {
            let pen = draw_with.lock().unwrap();
            let mut command_stack = cs_handle.lock().unwrap();

            if let Some(draw_command) = command_stack.pop() {
                pen.draw(cr, &draw_command);
            }
            Inhibit(false)
        });
    }

    pub fn set_mouse_move_event(&self, draw_with: Arc<Mutex<PenStream>>) {
        // Pointer motion mask.
        self.drawing_area.add_events(4);

        let cs_handle = self.shared_command_stack.clone();
        let mr_handle = self.shared_marked_rec.clone();
        self.drawing_area.connect_motion_notify_event(move |da: &DrawingArea,
                                                            event: &EventMotion| {
            let pen = draw_with.lock().unwrap();
            let mut command_stack = cs_handle.lock().unwrap();
            let mut marked_rec = mr_handle.lock().unwrap();

            let (mouse_x, mouse_y) = event.get_position();
            if let Some(rec_index) = pen.a_rec_surrounds(mouse_x, mouse_y) {
                if let None = *marked_rec {
                    command_stack.push(new_mark_c(rec_index, (0.0, 0.0, 255.0)));
                    // Make sure we only process marking a rectangle once.
                    *marked_rec = Some(rec_index);
                    queue_draw_of(rec_index, &pen, da);
                }
            } else if let Some(rec_index) = *marked_rec {
                *marked_rec = None;
                command_stack.push(DrawCommand::Draw(rec_index));
                queue_draw_of(rec_index, &pen, da);
            }
            Inhibit(false)
        });
    }

    pub fn set_size_change_event(&self, draw_with: Arc<Mutex<PenStream>>) {
        let cs_handle = self.shared_command_stack.clone();
        self.drawing_area.connect_size_allocate(move |da: &DrawingArea, _: &Allocation| {
            let mut pen = draw_with.lock().unwrap();
            let mut command_stack = cs_handle.lock().unwrap();
            demo3_size_change(pen.deref_mut(), da, command_stack.deref_mut());
        });
    }

    pub fn set_mouse_drag_event(&self, draw_with: Arc<Mutex<PenStream>>) {
        self.set_mouse_pressed_event();
        self.set_mouse_released_event(draw_with);
    }

    fn set_mouse_pressed_event(&self) {
        // Button press mask.
        self.drawing_area.add_events(256);

        let drag_start_pos_handle = self.shared_drag_start_pos.clone();
        self.drawing_area.connect_button_press_event(move |_: &DrawingArea, eb: &EventButton| {
            let mut drag_start_pos = drag_start_pos_handle.lock().unwrap();

            // On left press...
            if eb.as_ref().button == 1 {
                // ...store mouse position.
                let current_pos = eb.get_position();
                *drag_start_pos = Some((current_pos.0 as i32, current_pos.1 as i32));
            }
            Inhibit(false)
        });
    }

    fn set_mouse_released_event(&self, draw_with: Arc<Mutex<PenStream>>) {
        // Button press mask.
        self.drawing_area.add_events(512);

        let drag_start_pos_handle = self.shared_drag_start_pos.clone();
        let cs_handle = self.shared_command_stack.clone();
        self.drawing_area.connect_button_release_event(move |da: &DrawingArea,
                                                             eb: &EventButton| {
            let mut pen = draw_with.lock().unwrap();
            let mut command_stack = cs_handle.lock().unwrap();
            let drag_start_pos = drag_start_pos_handle.lock().unwrap();

            // On left release...
            if eb.as_ref().button == 1 {
                // ...draw new rectangle.
                let start_pos = drag_start_pos.expect("No speficied position found for where \
                                                      mouse drag started.");
                let current_pos = eb.get_position();
                let end_pos = (current_pos.0 as i32, current_pos.1 as i32);
                let width = end_pos.0 - start_pos.0;
                let height = end_pos.1 - start_pos.1;
                pen.add_rec_to_draw(start_pos.0 as f64,
                                    start_pos.1 as f64,
                                    width as f64,
                                    height as f64);
                let new_rec_index = pen.recs_len() - 1;
                command_stack.push(DrawCommand::Draw(new_rec_index));
                queue_draw_of(new_rec_index, &pen, da);
            }
            Inhibit(false)
        });
    }

    pub fn set_key_pressed_event(&self, draw_with: Arc<Mutex<PenStream>>) {
        self.drawing_area.set_can_focus(true);
        let cs_handle = self.shared_command_stack.clone();
        self.drawing_area.connect_grab_focus(move |_: &DrawingArea| {
                                                 let mut command_stack = cs_handle.lock().unwrap();
                                                 command_stack.push(DrawCommand::DrawAll);
                                             });

        let cs_handle = self.shared_command_stack.clone();
        let mr_handle = self.shared_marked_rec.clone();
        self.drawing_area.connect_key_release_event(move |da: &DrawingArea, event: &EventKey| {
            let mut pen = draw_with.lock().unwrap();
            let mut command_stack = cs_handle.lock().unwrap();
            let marked_rec = mr_handle.lock().unwrap();

            if let Some(rec_index) = *marked_rec {
                match event.get_keyval() {
                    65361 => {
                        // Left arrow key
                        // gdk::enums::key::leftarrow = 2299, get_keyval = 65361
                        demo2_key_release(true,
                                          rec_index,
                                          pen.deref_mut(),
                                          da,
                                          command_stack.deref_mut());
                    }
                    65363 => {
                        // Right arrow key
                        // gdk::enums::key::rightarrow = 2301, get_keyval = 65363
                        demo2_key_release(false,
                                          rec_index,
                                          pen.deref_mut(),
                                          da,
                                          command_stack.deref_mut());
                    }
                    _ => { /*Do nothing.*/ }
                }
            }
            Inhibit(false)
        });
    }
}

#[derive(Debug)]
pub enum DrawCommand {
    DrawAll,
    Draw(usize),
    Mark {
        shape_index: usize,
        colour: (f64, f64, f64),
    },
}

fn new_mark_c(i: usize, c: (f64, f64, f64)) -> DrawCommand {
    DrawCommand::Mark {
        shape_index: i,
        colour: c,
    }
}

fn queue_draw_of(ri: usize, pen: &MutexGuard<PenStream>, da: &DrawingArea) {
    // Trigger drawing area's draw event on surface...
    let (x, y, w, h) = pen.rectangle_info(ri);
    da.queue_draw_area(x - 1, y - 1, w + 2, h + 2);
}
