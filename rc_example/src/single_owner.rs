use crate::{create_registry, IoTRegistry};

struct IoTUseCase {
    registry: IoTRegistry
}

impl Drop for IoTUseCase {
    fn drop(&mut self) {
        println!("Drop IoTUseCase");
    }
}

pub fn single_owner_example() {
    let registry = create_registry();
    let use_case = IoTUseCase { registry };
    // let use_case_2 = IoTUseCase{registry}; // <-- Nope!

    // RC --> Reference Counting Smart Pointer (Read-only)

    use_case.registry.print_registry();
}
