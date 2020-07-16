use box_example::base_department::base_department_example;
use box_example::boxed_shapes::boxed_example;
use box_example::department::recursive_department_example;
use box_example::unboxed_shapes::unboxed_example;
use rc_example::multiple_owners::multiple_owners_example;
use rc_example::single_owner::single_owner_example;
use refcell_example::mutable_reference::mutable_reference_example;
use refcell_example::ref_cell::ref_cell_example;

fn main() {
    // Box - Owned trait objects
    unboxed_example();
    boxed_example();

    // Box - Recursive types
    base_department_example();
    recursive_department_example();

    // Rc - Multiple ownership
    single_owner_example();
    multiple_owners_example();

    // RefCell - Interior mutability
    mutable_reference_example();
    ref_cell_example();
}
