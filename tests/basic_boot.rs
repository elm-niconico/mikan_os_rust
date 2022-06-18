#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner_handler)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

use mikan_os_rust::{impl_deref_from_type, serial_println};
use mikan_os_rust::test_runner_handler;
use mikan_os_rust::usb::pci::configuration::tmp_find_usb_mouse_base;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    
    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("panic!");
    mikan_os_rust::test_panic_handler(info)
}


#[test_case]
fn should_find_base_bar() {
    let mmio_base = tmp_find_usb_mouse_base().unwrap_or(u64::MAX);
    assert_ne!(u64::MAX, mmio_base);
}


#[test_case]
pub fn should_impl_deref() {
    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
    struct A(u32);
    impl_deref_from_type!(A, u32);
    
    let a = A(32);
    serial_println!("{:?}", *a);
    assert_eq!(32, 32);
}



