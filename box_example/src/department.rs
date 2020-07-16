#[derive(Debug)]
struct Department {
    name: String,
    child: Option<Box<Department>>,
}

impl Department {
    fn new(name: String) -> Department {
        Department {
            name,
            child: None,
        }
    }
}

pub fn recursive_department_example() {
    let child = Department::new("Child".into());
    let mut parent = Department::new("Parent".into());
    parent.child = Some(Box::new(child));

    println!("Recursive type example: ");
    println!("{:?}", parent);
    println!("----------------------");
}
