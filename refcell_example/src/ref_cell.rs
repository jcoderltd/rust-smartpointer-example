use std::cell::RefCell;

// assume this trait comes from a library and you can't change the source code!
// but, you need to implement the same requirement!
trait LibraryPublisher {
    fn publish(&self, data: &str);
}

struct ConsolePublisher {
    published_count: RefCell<i64>
}

impl LibraryPublisher for ConsolePublisher {
    fn publish(&self, data: &str) {
        let mut count = self.published_count.borrow_mut();
        *count = *count + 1;
        println!("Published: {}", data);
    }
}

pub fn ref_cell_example() {
    println!("---------------");
    println!("Mutable Reference Example");
    let publisher = ConsolePublisher { published_count: RefCell::new(0) };
    publisher.publish("Example Data");
    println!("Published count: {}", *publisher.published_count.borrow());
    println!("---------------");
}

