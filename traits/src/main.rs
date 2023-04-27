use std::f32::consts::PI;

pub struct Square {
    pub length: f32,
}

pub struct Circle {
    pub radius: f32,
}

pub struct Triangle {
    pub base: f32,
    pub height: f32,
}

trait Shape {
    // Method must be defined
    fn area(&self) -> f32;
    // A default implementation exists. It leverages the mandatory method area
    fn volume(&self) -> f32 {
        // Just showing the default implementation
        self.area() * 2.0
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.length * self.length
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius.powi(2) * PI
    }
}

impl Shape for Triangle {
    fn area(&self) -> f32 {
        (self.height * self.base) / 2.0
    }
}

// packability accepts any type that implements Shape
// &impl Shape is syntactic sugar for the trait bound
//  fn packability<T: Shape>(shape: &T) -> f32 {
fn packability(shape: &impl Shape) -> f32 {
    shape.volume() / 5.0
}

fn main() {
    let s = Square { length: 3.0 };

    println!(
        "area: {}, volume: {}, packability: {}",
        s.area(),
        s.volume(),
        packability(&s)
    )
}
