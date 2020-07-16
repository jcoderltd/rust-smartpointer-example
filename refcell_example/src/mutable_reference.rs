trait Publisher {
    fn publish(&self, data: &str);
}

struct ConsolePublisher {
    published_count: i64
}

impl Publisher for ConsolePublisher {
    fn publish(&self, data: &str) {
        // self.published_count = self.published_count + 1;
        println!("Published: {}", data);
    }
}

pub fn mutable_reference_example() {
    println!("---------------");
    println!("Mutable Reference Example");
    let publisher = ConsolePublisher { published_count: 0 };
    publisher.publish("Example Data");
    println!("Published count: {}", publisher.published_count);
    println!("---------------");
}

