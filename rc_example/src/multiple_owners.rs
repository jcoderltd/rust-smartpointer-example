use std::rc::Rc;

use crate::{create_registry, IoTRegistry};

struct IoTUseCase {
    registry: Rc<IoTRegistry>
}

impl Drop for IoTUseCase {
    fn drop(&mut self) {
        println!("Drop IoTUseCase");
    }
}

pub fn multiple_owners_example() {
    println!("Start of outer block");

    let registry = create_registry();
    let use_case = IoTUseCase { registry: Rc::new(registry) };
    {
        println!("--------------------");
        println!("Start of inner block");
        let reg = use_case.registry.clone();
        let use_case_2 = IoTUseCase { registry: reg };
        use_case_2.registry.print_registry();
        println!("End of inner block");
        println!("--------------------");
    }
    println!("Back at outer block");

    use_case.registry.print_registry();
    println!("End of outer block");
}
