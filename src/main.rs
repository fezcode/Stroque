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

fn create_points_vector(p: Point, r: f32) -> Vec<LinePoint> {
    let mut point_vec: Vec<LinePoint> = Vec::new();

    for i in 1..20 {
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


fn check_if_within_range(p: Point, v: &mut Vec<(Point, f32)>, points_range:f32) -> bool {
    let mut is_valid = true;
    v.iter().enumerate().for_each(|(i, valid_point)| {
        // let t1 = p.x < valid_point.0.x - r*19 as f32;
        // let t2 = p.x > valid_point.0.x + valid_point.1*19 as f32;
        //
        // let t3 = p.y < valid_point.0.y - r*19 as f32;
        // let t4 = p.y > valid_point.0.y + valid_point.1*19 as f32;

        // let t1 = p.x < valid_point.0.x - r*19 as f32;               // left
        // let t2 = p.x > valid_point.0.x + valid_point.1*19 as f32;   // right
        //
        // let t3 = p.y > valid_point.0.y + r*19 as f32;               // top
        // let t4 = p.y < valid_point.0.y - valid_point.1*19 as f32;   // bottom

        if is_valid {
            let left   = valid_point.0.x - (points_range  * 20.0);
            let right  = valid_point.0.x + (valid_point.1 * 20.0);
            let top    = valid_point.0.y + (points_range  * 20.0);
            let bottom = valid_point.0.y - (valid_point.1 * 20.0);

            // let t_left = p.x < left;
            // let t_right = p.x > right;
            // let t_top = p.y > top;
            // let t_bottom = p.y < bottom;

            let t_left   = p.x > left;
            let t_right  = p.x < right;
            let t_top    = p.y < top;
            let t_bottom = p.y > bottom;

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

// (x,y)          (x+r,y)
//     .-----------.
//     |           |
//     |  PARAM    |
//     |           |
//     .-----------. (x+r, y+r)
// (x, y+r)

fn main() {
    // create a canvas to draw on
    let mut canvas = Canvas::new(4000, 4000);
    let mut valid_points: Vec<(Point, f32)> = Vec::new();

    // let mut drawing_vec: Vec<Drawing> = Vec::new();
    // let mut point_vec: Vec<LinePoint> = Vec::new();
    //
    // for x in 1..200 {
    //     point_vec.push(LinePoint::Straight {
    //         point: random_point()
    //     })
    // }

    // println!("{:#?}", point_vec);

    let mut rng = thread_rng();

    for y in (20..4000).step_by(20) {
        for x in (20..4000).step_by(20) {
            let mut start_point = Point { x: x as f32, y: y as f32};
            let mut size_r = rng.gen_range(10..20) as f32;
            let is_valid = check_if_within_range(start_point, &mut valid_points, 19.0);

            println!("=========================================================");
            println!("THE KING: {},{} | Size: {} | Valid: {}", x, y, size_r, is_valid);
            println!("=========================================================");

            if !is_valid {
                continue;
            }

            println!("ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ADDED ");

            valid_points.push((start_point, size_r));
            let vec = create_points_vector(start_point, size_r);
            let mut line: Drawing = Drawing::new();
            line = line.with_shape(Shape::Line {
                start: start_point,
                points: vec
            });
            line = line.with_style(Style::stroked(5, RGB { r: 10 * size_r as u8, g: 8 * size_r as u8, b: 6 * size_r as u8 } ));

            canvas.display_list.add(line);
        }
    }

    // for iter in 0..100 {
    //     let mut start = Point { x: 0.0 , y: 0.0 };
    //     let mut size_r = rng.gen_range(6..20) as f32;
    //     let mut is_valid = false;
    //     let mut try_count = 0;
    //
    //     println!("ITER: {}", iter);
    //     println!("SIZE_R: {}", size_r);
    //
    //     while !is_valid && try_count <= 200 {
    //         start = random_point();
    //         size_r = rng.gen_range(6..20) as f32;
    //         is_valid = check_if_within_range(start, &mut valid_points, size_r);
    //         try_count += 1;
    //     }
    //
    //     if try_count > 199 {
    //         continue;
    //     }
    //
    //     valid_points.push((start, size_r));
    //
    //     let vec = create_points_vector(start, size_r);
    //
    //     let mut line: Drawing = Drawing::new();
    //     line = line.with_shape(Shape::Line {
    //         start: start,
    //         points: vec
    //     });
    //     line = line.with_style(Style::stroked(5, RGB { r: 10 * size_r as u8, g: 8 * size_r as u8, b: 6 * size_r as u8 } ));
    //
    //     canvas.display_list.add(line);
    // }


// create a new drawing
//     let mut rect = Drawing::new()
// // give it a shape
//         .with_shape(Shape::Rectangle {
//             width: 50,
//             height: 50,
//         })
// // move it around
//         .with_xy(25.0, 25.0)
// // give it a cool style
//         .with_style(Style::stroked(5, RGB { r: 255, g: 0, b: 0 } ));

// add it to the canvas
//     canvas.display_list.add(rect);

// save the canvas as an svg
    render::save(
        &canvas,
        "tests/svg/basic_end_to_end.svg",
        SvgRenderer::new(),
    )
        .expect("Failed to save");
}
