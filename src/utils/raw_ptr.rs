use core::fmt::Debug;
use core::ptr;

use crate::usb::xhci::registers::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;


pub fn transmute_from_u64<T>(addr: u64) -> T {
    let ptr = addr as *const T;
    unsafe { ptr::read_volatile(ptr) }
}


pub fn transmute_register<T: Debug>(addr: u64) -> Volatile<T> {
    let register: T = transmute_from_u64::<T>(addr);
    let info = RegisterInfo::new(addr, register);
    Volatile::Core(info)
}

