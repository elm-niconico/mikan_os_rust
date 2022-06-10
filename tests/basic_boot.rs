#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner_handler)]
#![reexport_test_harness_main = "test_main"]


use core::mem;
use core::panic::PanicInfo;
use mikan_os_rust;
use mikan_os_rust::test_runner_handler;
use mikan_os_rust::usb::xhci::r::add_xhci;
use mikan_os_rust::usb::xhci::registers::capability::{StructuralParameters1, XhcParameters1};


#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    
    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mikan_os_rust::test_panic_handler(info)
}


#[test_case]
fn assert_one() {
    let param: u32 = 1000;
    
    let actual = unsafe{mem::transmute::<u32, XhcParameters1>(param)};
    
    assert_eq!(actual, add);
}



