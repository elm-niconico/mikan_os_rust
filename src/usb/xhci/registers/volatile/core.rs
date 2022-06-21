use core::fmt::Debug;

use crate::usb::xhci::registers::register_info::RegisterInfo;


pub fn read_volatile_by_core<T: Debug>(info: &RegisterInfo<T>) -> T {
    unsafe { core::ptr::read_volatile(info.ptr()) }
}


pub fn write_volatile_by_core<T: Debug>(info: &RegisterInfo<T>, new_register: T) {
    unsafe {
        core::ptr::write_volatile(info.ptr(), new_register);
    }
}

