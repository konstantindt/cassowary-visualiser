use gtk::prelude::*;
use gtk::DrawingArea;
use cairo::Context;
use pen::PenStream;

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

    pub fn set_draw_callback(&self, to_draw: PenStream) {
        self.drawing_area.connect_draw(move |_: &DrawingArea, cr: &Context| {
            to_draw.draw_all_recs(cr);
            Inhibit(false)
        });
    }
}
