use ::std::f64::consts::PI;
use ::gtk::prelude::*;
use ::gtk::DrawingArea;
use ::cairo::Context;

pub struct Visualiser {
    drawing_area: DrawingArea,
}

impl Visualiser {
    pub fn new() -> Visualiser {
        Visualiser { drawing_area: DrawingArea::new() }
    }

    pub fn get_drawing_area(&self) -> &DrawingArea {
        &self.drawing_area
    }

    pub fn set_draw_callback(&self) {
        self.drawing_area.connect_draw(|d_a: &DrawingArea, cr: &Context| {
            let d_a_width: f64 = d_a.get_allocated_width() as f64;
            let d_a_height: f64 = d_a.get_allocated_height() as f64;
            cr.arc(d_a_width / 2.0,
                   d_a_height / 2.0,
                   d_a_width.min(d_a_height) / 2.0,
                   0.0,
                   2.0 * PI);
            cr.set_source_rgba(0.0, 0.0, 255.0, 1.0);
            cr.stroke();
            Inhibit(false)
        });
    }
}
