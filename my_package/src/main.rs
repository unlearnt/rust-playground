// mod my_module;

mod my_lib {
    pub mod module_a {
        pub fn foo() {
            println!("Function foo from module_a.");
        }
    }
    pub mod module_b {
        pub fn bar() {
            println!("Function bar from module_b.");
        }
    }
}

mod my_module {
    pub struct MyStruct{
        pub field: i32,
    }
}

use crate::my_module::MyStruct;
use my_lib::module_a::foo;

fn main() {
    // my_module::foo();
    foo();

    let instance = MyStruct { field: 123 };
    println!("Field value: {}", instance.field);
}
