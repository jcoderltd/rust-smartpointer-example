pub mod single_owner;
pub mod multiple_owners;

/// An immutable IoTRegistry - once created it should be read-only to all users that require access
/// to the registry!
struct IoTRegistry {
    register: Vec<Box<dyn Device>>
}

impl IoTRegistry {
    fn print_registry(&self) {
        println!("IoT Devices: ");
        self.register.iter().for_each(|d| d.print_details());
        println!("---------------");
    }
}

trait Device {
    fn print_details(&self);
}

struct Light {
    name: String,
}

impl Device for Light {
    fn print_details(&self) {
        println!("Light: {}", self.name);
    }
}

struct RemoteSwitch {
    name: String,
}

impl Device for RemoteSwitch {
    fn print_details(&self) {
        println!("RemoteSwitch: {}", self.name);
    }
}

fn create_registry() -> IoTRegistry {
    let mut r = IoTRegistry { register: Vec::new() };
    r.register.push(Box::new(Light { name: "Bedroom Light".into() }));
    r.register.push(Box::new(Light { name: "Kitchen Light".into() }));
    r.register.push(Box::new(RemoteSwitch { name: "Bedroom Switch".into() }));
    r
}

impl Drop for IoTRegistry {
    fn drop(&mut self) {
        println!("Drop IoTRegistry");
    }
}