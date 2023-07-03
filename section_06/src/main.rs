use std::fmt::Debug;

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

#[derive(Debug)]
struct Motorcycle {
    mpg: i32,
    color: Color,
    top_speed: i32,
}

trait Vehicle {
    fn new(mpg: i32, color: Color, top_speed: i32) -> Self;

    fn set_mpg(&mut self, mpg: i32);

    fn set_color(&mut self, color: Color);

    fn set_top_speed(&mut self, top_speed: i32);
}

impl Vehicle for Car {
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

impl Vehicle for Motorcycle {
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

fn print_vehicle<T: Debug>(input: &T) {
    println!("{:?}", input);
}

fn main() {
    let mut car = Car::new(100, Color::Gray, 270);

    print_vehicle(&car);

    car.set_mpg(80);
    car.set_color(Color::Navy);
    car.set_top_speed(300);

    print_vehicle(&car);

    let mut motorcycle = Motorcycle::new(50, Color::Red, 120);

    print_vehicle(&motorcycle);

    motorcycle.set_mpg(75);
    motorcycle.set_color(Color::Green);
    motorcycle.set_top_speed(140);

    print_vehicle(&motorcycle);
}
