use draw::*;
use draw::Point;
use draw::shape::LinePoint;
use rand::{thread_rng, Rng};

fn random_point() -> Point {
    let mut rng = thread_rng();
    let x = rng.gen_range(0..2000);
    let y = rng.gen_range(0..2000);
    Point {
        x: x as f32,
        y: y as f32
    }
}

fn create_points_vector(p: Point) -> Vec<LinePoint> {
    let mut point_vec: Vec<LinePoint> = Vec::new();

    for x in 1..10 {
        let p1 = Point { x: p.x,       y: p.y };
        let p2 = Point { x: p.x + i*8, y: p.y };
        let p3 = Point { x: p.x + i*8, y: p.y + i*8};
        let p4 = Point { x: p.x,       y: p.y + i*8};

        point_vec.push(LinePoint::Straight { point: p1 });
        point_vec.push(LinePoint::Straight { point: p2 });
        point_vec.push(LinePoint::Straight { point: p3 });
        point_vec.push(LinePoint::Straight { point: p4 });
    };

    point_vec
}

// (x,y)          (x+r,y)
//     .-----------.
//     |           |
//     |           |
//     |           |
//     .-----------. (x+r, y+r)
// (x, y+r)

fn main() {
    // create a canvas to draw on
    let mut canvas = Canvas::new(2000, 2000);

    // let mut drawing_vec: Vec<Drawing> = Vec::new();
    // let mut point_vec: Vec<LinePoint> = Vec::new();
    //
    // for x in 1..200 {
    //     point_vec.push(LinePoint::Straight {
    //         point: random_point()
    //     })
    // }

    // println!("{:#?}", point_vec);

    let mut line: Drawing = Drawing::new();
    line = line.with_shape(Shape::Line {
        start: random_point(),
        points: point_vec
    });
    line = line.with_style(Style::stroked(5, RGB { r: 255, g: 0, b: 0 } ));


// create a new drawing
    let mut rect = Drawing::new()
// give it a shape
        .with_shape(Shape::Rectangle {
            width: 50,
            height: 50,
        })
// move it around
        .with_xy(25.0, 25.0)
// give it a cool style
        .with_style(Style::stroked(5, RGB { r: 255, g: 0, b: 0 } ));

// add it to the canvas
//     canvas.display_list.add(rect);
     canvas.display_list.add(line);

// save the canvas as an svg
    render::save(
        &canvas,
        "tests/svg/basic_end_to_end.svg",
        SvgRenderer::new(),
    )
        .expect("Failed to save");
}
