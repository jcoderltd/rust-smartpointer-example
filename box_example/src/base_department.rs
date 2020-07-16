#[derive(Debug)]
struct Department {
    name: String,
}

impl Department {
    fn new(name: String) -> Department {
        Department {
            name,
        }
    }
}

pub fn base_department_example() {
    let child = Department::new("Child".into());
    let parent = Department::new("Parent".into());

    println!("Base department example: ");
    println!("{:?}", child);
    println!("{:?}", parent);
    println!("----------------------");
}
