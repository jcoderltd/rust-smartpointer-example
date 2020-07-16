use crate::shapes::{Circle, Drawable, Square};

struct Scene<T> where T: Drawable {
    objects: Vec<T>,
}

impl<T: Drawable> Drawable for Scene<T> {
    fn draw(&self) {
        println!("Unboxed Scene");
        // T - Square
        // Metodo Square::draw (direccion en memoria)
        self.objects.iter().for_each(|x| x.draw());
        println!("-------------");
    }
}

pub fn unboxed_example() {
    let s = Square { size: 2.0 };
    let c = Circle { radius: 1.0 };

    let mut scene1 = Scene {
        objects: Vec::new(),
    };
    scene1.objects.push(s);
    // scene1.objects.push(c); // <-- Nope!

    let scene2 = Scene {
        objects: vec!(c),
    };

    scene1.draw();
    scene2.draw();
}
