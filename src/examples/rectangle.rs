#![allow(dead_code)]
#[derive(Debug)]

struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left : Point,
    bottom_right : Point,
}

fn main()
{    
    let point: Point = Point { x: 10.3, y: 0.4 };
    
    let bottom_right = Point { x: 5.2, ..point };

    let Point { x: left_edge, y: top_edge } = point;

    let rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge },
        bottom_right : bottom_right,
    };

    println!("{:?}", rect_area(rectangle));
}

fn rect_area(rectangle: Rectangle) -> f32
{
    let Rectangle {
        top_left: Point {x: x1, y: y1},
        bottom_right: Point {x: x2, y: y2},
    } = rectangle;
    
    let dx = x1 - x2;
    let dy = if y2 > y1 { dy = y2 - y1 } else { dy = y1 };

    let dx = dx.abs();
    println!("{:?}, {:?}", dx, y1);

    return dx * dy;
}