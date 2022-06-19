use core::fmt::Debug;

use crate::usb::xhci::registers::register_info::RegisterInfo;


pub fn read_volatile_by_core<T: Debug>(register_info: &RegisterInfo<T>) -> T {
    unsafe { core::ptr::read_volatile(register_info.get_register_raw_ptr()) }
}


pub fn write_volatile_by_core<T: Debug>(register_info: &RegisterInfo<T>, new_register: T) {
    unsafe {
        core::ptr::write_volatile(register_info.get_register_raw_ptr(), new_register);
    }
}

