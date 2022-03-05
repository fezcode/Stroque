use draw::*;
use draw::Point;
use draw::shape::LinePoint;
use rand::{thread_rng, Rng};


fn random_point() -> Point {
    let mut rng = thread_rng();
    let x = rng.gen_range(20..3980);
    let y = rng.gen_range(20..3980);
    Point {
        x: x as f32,
        y: y as f32
    }
}

fn create_points_vector(p: Point, r: f32, loop_size: i32) -> Vec<LinePoint> {
    let mut point_vec: Vec<LinePoint> = Vec::new();

    for i in 0..loop_size {
        let mut p1 = Point { x: p.x,                 y: p.y };
        let mut p2 = Point { x: p.x + (r*i as f32) , y: p.y };
        let mut p3 = Point { x: p.x + (r*i as f32) , y: p.y + (r*i as f32) };
        let mut p4 = Point { x: p.x,                 y: p.y + (r*i as f32) };

        // if random_number > 5 {
        //     // rotate
        //     p1.x = -1.0 * (p1.y); p1.y = 1.0 * (p1.x);
        //     p2.x = -1.0 * (p2.y); p2.y = 1.0 * (p2.x);
        //     p3.x = -1.0 * (p3.y); p3.y = 1.0 * (p3.x);
        //     p4.x = -1.0 * (p4.y); p4.y = 1.0 * (p4.x);
        // }

        // push
        point_vec.push(LinePoint::Straight { point: p1 });
        point_vec.push(LinePoint::Straight { point: p2 });
        point_vec.push(LinePoint::Straight { point: p3 });
        point_vec.push(LinePoint::Straight { point: p4 });

        // println!("------------- {} ----------------", i);
        // println!("{:?}", p1);
        // println!("{:?}", p2);
        // println!("{:?}", p3);
        // println!("{:?}", p4);
        // println!("---------------------------");

    };

    point_vec
}


fn check_if_within_range(p: Point, v: &mut Vec<(Point, f32, f32)>, points_range:f32, loop_size: f32) -> bool {
    let mut is_valid = true;
    v.iter().rev().take(1000).enumerate().for_each(|(i, valid_point)| {
        if is_valid {
            let left   = valid_point.0.x - (points_range  * loop_size);
            let right  = valid_point.0.x + (valid_point.1 * valid_point.2);
            let top    = valid_point.0.y - (points_range  * loop_size);
            let bottom = valid_point.0.y + (valid_point.1 * valid_point.2);
            
            let t_left   = p.x > left;
            let t_right  = p.x < right;
            let t_top    = p.y > top;
            let t_bottom = p.y < bottom;

            // println!("-----------Trying VP {}--------------", i);
            // // println!("l:{} | r:{} | t: {} | b: {}", left, right, top, bottom);
            // println!("P:{:?} | VP:{:?} | [{:?} {:?} {:?} {:?}] | Range: {} | VP: {}", p,valid_point.0,t_left,t_right,t_top,t_bottom,r*19.0, valid_point.1*19.0);
            // println!("---------------------------");

            is_valid = is_valid && ( !(t_left && t_right  && t_top && t_bottom) );

            // println!("IS IT OKAY: {}", is_valid);
        }

    });
    return is_valid;
}

fn set_line_and_loop(size_r: f32) -> (u32, i32) {
    let mut line_width: u32 = 0;
    let mut loop_size: i32 = 0;

    if size_r < 4.0 {
        line_width = 1;
        loop_size = 3;
    } else if size_r < 6.0 {
        line_width = 1;
        loop_size = 4;
    } else if size_r < 8.0 {
        line_width = 2;
        loop_size = 5;
    } else if size_r < 11.0 {
        line_width = 3;
        loop_size = 8;
    } else if size_r < 14.0 {
        line_width = 4;
        loop_size = 10;
    } else if size_r < 18.0 {
        line_width = 5;
        loop_size = 14;
    } else {
        line_width = 5;
        loop_size = 20;
    }

    return (line_width, loop_size);
}

fn randomize_color(size_r: f32) -> RGB {
    let mut rng = thread_rng();
    let r = /* rng.gen_range(210..255) - */ (10 * size_r as u8);
    let g = /* rng.gen_range(210..255) - */ (8  * size_r as u8);
    let b = /* rng.gen_range(210..255) - */ (9  * size_r as u8);
    RGB {
        r,
        g,
        b
    }
    // RGB {
    //     r: 255 - (12 * size_r as u8),
    //     g: 255 - (9 * size_r as u8),
    //     b: 255 - (7 * size_r as u8)
    // }
}

// (0,0)------------------------------->
// |
// |  (x,y)          (x+r,y)
// |      .-----------.
// |      |           |
// |      |  PARAM    |
// |      |           |
// |      .-----------. (x+r, y+r)
// |  (x, y+r)
// V
fn main() {
    // create a canvas to draw on
    let mut canvas = Canvas::new(2000, 2000);
    let mut valid_points: Vec<(Point, f32, f32)> = Vec::new();
    let mut rng = thread_rng();

    for y in (20..2000).step_by(4) {

        println!("y: {} | VecSize: {}", y, valid_points.len());

        for x in (20..2000).step_by(4) {
            let mut start_point = Point { x: x as f32, y: y as f32};
            let mut size_r = rng.gen_range(10..20) as f32;
            // let mut is_valid = check_if_within_range(start_point, &mut valid_points, 10.0);
            let mut line_width = 1;
            let mut loop_size = 20;
            let mut is_valid = check_if_within_range(start_point, &mut valid_points, size_r, loop_size as f32);

            if !is_valid {
                size_r = 20.0;
                while !is_valid && size_r > 3.0 {
                    (line_width, loop_size) = set_line_and_loop(size_r);
                    is_valid = check_if_within_range(start_point, &mut valid_points, size_r, loop_size as f32);
                    size_r -= 1.0;
                }
            }

            if is_valid {
                (line_width, loop_size) = set_line_and_loop(size_r);
            } else {
                continue;
            }

            valid_points.push((start_point, size_r, loop_size as f32));
            let vec = create_points_vector(start_point, size_r, loop_size);
            let mut line: Drawing = Drawing::new();
            line = line.with_shape(Shape::Line {
                start: start_point,
                points: vec
            });
            line = line.with_style(Style::stroked(line_width, randomize_color(size_r) ));

            canvas.display_list.add(line);

        }

    }
    render::save(&canvas,
        "tests/svg/test.svg",
        SvgRenderer::new(),
    ).expect("Failed to save");
}
