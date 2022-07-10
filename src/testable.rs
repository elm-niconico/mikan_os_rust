use core::any::type_name;

use crate::{serial_print, serial_println};

pub trait Testable {
    fn run(&self);
}


impl<T> Testable for T where T: Fn() {
    fn run(&self) {
        let fn_name = type_name::<T>();
        serial_print!("test {}....\t", fn_name);
        self();
        serial_println!("[ok]");
    }
}