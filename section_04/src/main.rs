#[derive(Debug)]
enum Color {
    Red,
    Black,
    Navy,
    Gray,
    Green,
}

#[derive(Debug)]
struct Car {
    mpg: i32,
    color: Color,
    top_speed: i32,
}

impl Car {
    fn new(mpg: i32, color: Color, top_speed: i32) -> Self {
        Self {
            mpg,
            color,
            top_speed,
        }
    }

    fn set_mpg(&mut self, mpg: i32) {
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn set_top_speed(&mut self, top_speed: i32) {
        self.top_speed = top_speed;
    }
}

fn main() {
    let mut car = Car::new(100, Color::Gray, 270);

    println!("{:?}", car);
    
    car.set_mpg(80);
    car.set_color(Color::Navy);
    car.set_top_speed(300);

    println!("{:?}", car);
}
