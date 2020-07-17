use crate::shapes::{Circle, Drawable, Square};

struct Scene {
    objects: Vec<Box<dyn Drawable>>,
}

impl Drawable for Scene {
    fn draw(&self) {
        println!("Boxed Scene");
        self.objects.iter().for_each(|x| x.draw());
        println!("-------------");
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: Vec::new() }
    }

    pub fn add_scene_object(&mut self, object: Box<dyn Drawable>) {
        self.objects.push(object);
    }
}

pub fn boxed_example() {
    let s = Square { size: 2.0 };
    let c = Circle { radius: 1.0 };

    let mut scene1 = Scene::new();
    scene1.add_scene_object(Box::new(s));
    scene1.add_scene_object(Box::new(c));

    let mut scene2 = Scene::new();
    scene2.add_scene_object(Box::new(Square { size: 3.0 }));

    scene1.draw();
    scene2.draw();
}
