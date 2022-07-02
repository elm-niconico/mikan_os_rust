use core::fmt::Debug;
use core::ptr;

use crate::usb_my::xhci::registers::register_info::RegisterInfo;
use crate::usb_my::xhci::registers::volatile::Volatile;

pub fn transmute_from_u64<T>(addr: u64) -> T {
    let ptr = addr as *const T;
    unsafe { ptr::read_volatile(ptr) }
}

pub fn transmute_register<T: Debug>(addr: u64) -> Volatile<T> {
    let info = RegisterInfo::new(addr);
    Volatile::Core(info)
}