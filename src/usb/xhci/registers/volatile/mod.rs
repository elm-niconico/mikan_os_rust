use ::core::fmt::Debug;

use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::usb::xhci::registers::volatile::core::{read_volatile_by_core, write_volatile_by_core};

mod core;
pub mod register_operators;

pub trait VolatileRegister<T> {
    fn read(&self) -> T;
    fn write(&mut self, src: T);
    fn update(&mut self, update_fn: fn(r: &mut T));
}

#[derive(Debug, Clone, Copy)]
pub enum Volatile<T: Debug> {
    Core(RegisterInfo<T>),
}

impl<T: Debug> VolatileRegister<T> for Volatile<T> {
    fn read(&self) -> T {
        match self {
            Volatile::Core(info) => read_volatile_by_core(info),
        }
    }

    fn write(&mut self, new_register: T) {
        match self {
            Volatile::Core(register_info) => write_volatile_by_core(register_info, new_register),
        }
    }

    fn update(&mut self, update_fn: fn(&mut T)) {
        let mut r = self.read();
        update_fn(&mut r);
        self.write(r);
    }
}