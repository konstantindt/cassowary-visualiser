use ::cairo::Context;

pub struct PenStream {
    recs_to_draw: Vec<Rectangle>,
}

impl PenStream {
    pub fn new() -> PenStream {
        PenStream { recs_to_draw: Vec::new() }
    }

    pub fn add_rec_to_draw(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.recs_to_draw.push(Rectangle {
            x_pos: x,
            y_pos: y,
            width: w,
            height: h,
        });
    }

    pub fn draw_all_recs(&self, cr: &Context) {
        for r in self.recs_to_draw.iter() {
            cr.rectangle(r.x_pos, r.y_pos, r.width, r.height);
        }
        cr.stroke();
    }
}

struct Rectangle {
    x_pos: f64,
    y_pos: f64,
    width: f64,
    height: f64,
}
