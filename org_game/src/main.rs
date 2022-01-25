use std::string::String;
use std::vec::Vec;

fn main() {
    struct Org {
        employees: Vec<Employee>;
        num_employees: i32;
    }

    struct Employee {
        age: u8;
        gender: String;
    }
}
