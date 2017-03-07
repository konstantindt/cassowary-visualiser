use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use gtk::prelude::*;
use gtk::DrawingArea;
use gdk::EventMotion;
use cairo::Context;
use pen::PenStream;

pub struct Visualiser {
    drawing_area: DrawingArea,
    shared_marked_rec: Arc<Mutex<Option<usize>>>,
}

impl Visualiser {
    pub fn new() -> Visualiser {
        Visualiser {
            drawing_area: DrawingArea::new(),
            shared_marked_rec: Arc::new(Mutex::new(None)),
        }
    }

    pub fn drawing_area(&self) -> &DrawingArea {
        &self.drawing_area
    }

    pub fn set_draw_event(&self, to_draw: Arc<Mutex<PenStream>>) {
        let shared_marked_rec = self.shared_marked_rec.clone();
        self.drawing_area.connect_draw(move |_: &DrawingArea, cr: &Context| {
            let pen = to_draw.lock().unwrap();
            let marked_rec = shared_marked_rec.lock().unwrap();

            pen.draw_all_recs(cr, marked_rec.deref());
            Inhibit(false)
        });
    }

    pub fn set_mouse_move_event(&self, recs_shared: Arc<Mutex<PenStream>>) {
        self.drawing_area.add_events(4);

        let shared_marked_rec = self.shared_marked_rec.clone();
        self.drawing_area
            .connect_motion_notify_event(move |da: &DrawingArea, event: &EventMotion| {
                let recs = recs_shared.lock().unwrap();
                let mut marked_rec = shared_marked_rec.lock().unwrap();

                let (mouse_x, mouse_y) = event.get_position();
                if let Some(i) = recs.a_rec_surrounds(mouse_x, mouse_y) {
                    if let None = *marked_rec {
                        *marked_rec = Some(i);
                        queue_draw_of(i, &recs, da);
                    }
                } else if let Some(i) = *marked_rec {
                    *marked_rec = None;
                    queue_draw_of(i, &recs, da);
                }
                Inhibit(false)
            });
    }
}

fn queue_draw_of(index: usize, recs: &MutexGuard<PenStream>, da: &DrawingArea) {
    let (x, y, w, h) = recs.rectangle_info(index);
    da.queue_draw_area(x - 1, y - 1, w + 2, h + 2);
}
