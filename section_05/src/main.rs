enum Shape {
    TRIANGLE,
    SQUARE,
    PENTAGON,
    OCTAGON,
}

impl Shape {
    fn corners(&self) -> i32 {
        match self {
            Shape::TRIANGLE => 3,
            Shape::SQUARE => 4,
            Shape::PENTAGON => 5,
            Shape::OCTAGON => 8,
        }
    }
}

fn main() {
    let triangle = Shape::TRIANGLE;
    let square = Shape::SQUARE;
    let pentagon = Shape::PENTAGON;
    let octagon = Shape::OCTAGON;
    
    println!("{}", triangle.corners());
    println!("{}", square.corners());
    println!("{}", pentagon.corners());
    println!("{}", octagon.corners());
}
