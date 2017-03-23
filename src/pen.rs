use cairo::Context;
use visualiser::DrawCommand;

pub struct PenStream {
    recs_to_draw: Vec<Rectangle>,
}

impl PenStream {
    pub fn new() -> PenStream {
        PenStream { recs_to_draw: Vec::new() }
    }

    pub fn rectangle_info(&self, for_rec: usize) -> (i32, i32, i32, i32) {
        let ref r = self.recs_to_draw[for_rec];
        (r.x_pos as i32, r.y_pos as i32, r.width as i32, r.height as i32)
    }

    pub fn a_rec_surrounds(&self, x: f64, y: f64) -> Option<usize> {
        for (i, rec) in self.recs_to_draw.iter().enumerate() {
            if rec.surrounds(x, y) {
                return Some(i);
            }
        }
        None
    }

    pub fn draw(&self, cr: &Context, dc: &DrawCommand) {
        match dc {
            &DrawCommand::Draw(shape_index) => {
                let ref r = self.recs_to_draw[shape_index];
                cr.rectangle(r.x_pos, r.y_pos, r.width, r.height);
                cr.stroke();
            }
            &DrawCommand::Mark { shape_index, colour } => {
                let ref r = self.recs_to_draw[shape_index];
                cr.set_source_rgb(colour.0, colour.1, colour.2);
                cr.rectangle(r.x_pos, r.y_pos, r.width, r.height);
                cr.stroke();
                // Return to default colour (Black).
                cr.set_source_rgb(0.0, 0.0, 0.0);
            }
            &DrawCommand::DrawAll => {
                for r in self.recs_to_draw.iter() {
                    cr.rectangle(r.x_pos, r.y_pos, r.width, r.height);
                }
                cr.stroke();
            }
        }
    }

    pub fn add_rec_to_draw(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.recs_to_draw.push(Rectangle {
                                   x_pos: x,
                                   y_pos: y,
                                   width: w,
                                   height: h,
                               });
    }

    pub fn clear_all_recs(&mut self) {
        self.recs_to_draw.clear();
    }
}

struct Rectangle {
    x_pos: f64,
    y_pos: f64,
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn surrounds(&self, x: f64, y: f64) -> bool {
        self.x_pos < x && x - self.x_pos < self.width && self.y_pos < y &&
        y - self.y_pos < self.height
    }
}
