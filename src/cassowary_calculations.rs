use gtk::{WidgetExt, DrawingArea};
use cassowary;
use cassowary::math::variables::{new_var, new_const};
use cassowary::math::relationships::Relationship;
use cassowary::math::expressions::Expression;
use cassowary::objective::functions::Function;
use cassowary::objective::problems::ProblemType;
use cassowary::objective::constraints::{new_reg_con, SystemOfConstraints};
use pen::PenStream;
use visualiser::DrawCommand;

pub fn cal_demo1(da_width: f32, da_height: f32, pen: &mut PenStream) {
    let x_loc = 30.0;
    let y_loc = 20.0;
    let side_margin = 30.0;
    let top_margin = 20.0;
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

pub fn cal_demo2(x_loc1: f32,
                 y_loc1: f32,
                 x_loc2: f32,
                 y_loc2: f32,
                 side_margin: f32,
                 mid_margin: f32,
                 top_margin: f32,
                 da_width: f32,
                 da_height: f32,
                 pen: &mut PenStream) {
    let exp1 = Expression::new(vec![new_var("P", 1.0)],
                               Relationship::EQ,
                               vec![new_var("width1", 0.5),
                                    new_var("height1", 1.0),
                                    new_var("width2", 1.0),
                                    new_var("height2", 1.0)]);
    let exp2 = Expression::new(vec![new_var("width1", 1.0), new_var("width2", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con1",
                                              da_width - (2.0 * side_margin) - mid_margin)]);
    let exp3 = Expression::new(vec![new_var("height1", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con2", da_height - 2.0 * top_margin)]);
    let exp4 = Expression::new(vec![new_var("height2", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con3", da_height - 2.0 * top_margin)]);
    let exp5 = Expression::new(vec![new_var("width1", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con4", da_width - x_loc1)]);
    let exp6 = Expression::new(vec![new_var("width1", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con4", x_loc2 - x_loc1 - mid_margin)]);
    let exp7 = Expression::new(vec![new_var("width2", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con4", da_width - (x_loc2 + side_margin))]);
    let exp8 = Expression::new(vec![new_var("height1", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con5", da_height - y_loc1)]);
    let exp9 = Expression::new(vec![new_var("width2", 1.0)],
                               Relationship::LEQ,
                               vec![new_const("con6", da_width - x_loc2)]);
    let exp10 = Expression::new(vec![new_var("height2", 1.0)],
                                Relationship::LEQ,
                                vec![new_const("con7", da_height - y_loc2)]);
    let mut objective_func = Function::new(exp1, ProblemType::MAX);
    let c1 = new_reg_con(exp2);
    let c2 = new_reg_con(exp3);
    let c3 = new_reg_con(exp4);
    let c4 = new_reg_con(exp5);
    let c5 = new_reg_con(exp6);
    let c6 = new_reg_con(exp7);
    let c7 = new_reg_con(exp8);
    let c8 = new_reg_con(exp9);
    let c9 = new_reg_con(exp10);
    let subject_to = SystemOfConstraints::new(vec![c1, c2, c3, c4, c5, c6, c7, c8, c9]);
    let solution = cassowary::optimise(&mut objective_func, &subject_to);
    let width1 = solution.iter()
        .find(|&entry| entry.0 == "width1")
        .unwrap()
        .1;
    let height1 = solution.iter()
        .find(|&entry| entry.0 == "height1")
        .unwrap()
        .1;
    let width2 = solution.iter()
        .find(|&entry| entry.0 == "width2")
        .unwrap()
        .1;
    let height2 = solution.iter()
        .find(|&entry| entry.0 == "height2")
        .unwrap()
        .1;
    println!("width1 = {}\nheight1 = {}\n", width1, height1);
    println!("width2 = {}\nheight2 = {}", width2, height2);
    pen.add_rec_to_draw(x_loc1 as f64, y_loc1 as f64, width1 as f64, height1 as f64);
    pen.add_rec_to_draw(x_loc2 as f64, y_loc2 as f64, width2 as f64, height2 as f64);
}

pub fn demo2_key_release(left: bool,
                         ri: usize,
                         pen: &mut PenStream,
                         da: &DrawingArea,
                         cs: &mut Vec<DrawCommand>) {
    let (x, y) = match left {
        true => {
            let rec_info = pen.rectangle_info(ri);
            (rec_info.0 - 5, rec_info.1)
        }
        false => {
            let rec_info = pen.rectangle_info(ri);
            (rec_info.0 + 5, rec_info.1)
        }
    };

    match ri {
        0 => {
            let (da_width, da_height) = drawing_area_height_width(da);
            let (x2, y2, _, _) = pen.rectangle_info(1);
            pen.clear_all_recs();
            cal_demo2(x as f32,
                      y as f32,
                      x2 as f32,
                      y2 as f32,
                      30.0,
                      20.0,
                      20.0,
                      da_width as f32,
                      da_height as f32,
                      pen);
            draw_demo2_key_release(x - 5, y, da_width, da_height, da, cs);
        }
        _ => {
            let (da_width, da_height) = drawing_area_height_width(da);
            let (x1, y1, _, _) = pen.rectangle_info(0);
            pen.clear_all_recs();
            cal_demo2(x1 as f32,
                      y1 as f32,
                      x as f32,
                      y as f32,
                      30.0,
                      20.0,
                      20.0,
                      da_width as f32,
                      da_height as f32,
                      pen);
            draw_demo2_key_release(x1, y1, da_width, da_height, da, cs);
        }
    }
}

fn draw_demo2_key_release(x: i32,
                          y: i32,
                          da_w: i32,
                          da_h: i32,
                          da: &DrawingArea,
                          cs: &mut Vec<DrawCommand>) {
    cs.push(DrawCommand::DrawAll);
    da.queue_draw_area(x - 1, y - 1, da_w - 58, da_h - 38);
}

fn drawing_area_height_width(da: &DrawingArea) -> (i32, i32) {
    (da.get_allocated_width(), da.get_allocated_height())
}
