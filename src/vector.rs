pub struct Point {
    pub x : f64,
    pub y : f64,
}

pub struct Point3 {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

pub fn sum_point(a: Point, b: Point) -> Point {
    Point { x: a.x + b.x, y: a.y + b.y }
}