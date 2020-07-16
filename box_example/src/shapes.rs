pub trait Drawable {
    fn draw(&self);
}

#[derive(Debug)]
pub struct Square {
    pub size: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("{:?}", self);
    }
}
