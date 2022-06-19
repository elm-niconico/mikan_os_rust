use ::core::fmt::Debug;

use crate::usb::xhci::registers::volatile::core::{
    read_volatile_by_core, write_volatile_by_core,
};
use crate::usb::xhci::registers::register_info::RegisterInfo;

mod core;
pub mod register_operators;

pub trait VolatileRegister<T> {
    fn read_volatile(&self) -> T;
    fn write_volatile(&mut self, src: T);
    fn update_volatile(&mut self, update_fn: fn(r: &mut T));
}

#[derive(Debug, Clone, Copy)]
pub enum Volatile<T: Debug> {
    Core(RegisterInfo<T>),
}

impl<T: Debug> VolatileRegister<T> for Volatile<T> {
    fn read_volatile(&self) -> T {
        match self {
            Volatile::Core(register_info) => read_volatile_by_core(register_info),
        }
    }

    fn write_volatile(&mut self, new_register: T) {
        match self {
            Volatile::Core(register_info) => write_volatile_by_core(register_info, new_register),
        }
    }

    fn update_volatile(&mut self, update_fn: fn(&mut T)) {
        let mut r = self.read_volatile();
        update_fn(&mut r);
        self.write_volatile(r);
    }
}
