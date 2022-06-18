#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner_handler)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

use mikan_os_rust::{impl_deref_from_type, serial_println};
use mikan_os_rust::test_runner_handler;
use mikan_os_rust::usb::pci::configuration::tmp_find_usb_mouse_base;
use mikan_os_rust::usb::xhci::registers::capability::structs::capability_register::CapabilityRegister;
use mikan_os_rust::usb::xhci::registers::capability::create::register_creator::ICapabilityRegisterCreate;
use mikan_os_rust::usb::xhci::registers::create_type::CreateType;
use mikan_os_rust::usb::xhci::registers::read_write::volatile::IVolatile;


static OFFSET: u64 = 1649267441664;


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


pub fn extract_usb_cmd_base_addr() -> u64 {
    let mmio = extract_virtual_mmio_base_addr();
    let cap = extract_cap_register(mmio);
    let cap_len: u8 = cap.cap_length.into();
    mmio + cap_len as u64
}


fn extract_virtual_mmio_base_addr() -> u64 {
    let mmio_base = tmp_find_usb_mouse_base().unwrap();
    mmio_base + OFFSET
}


fn extract_cap_register(mmio_base: u64) -> CapabilityRegister {
    let create = CreateType::UncheckTransmute;
    let volatile = create
        .new_capability(mmio_base)
        .expect("Failed Mapping to Cap Register");
    
    volatile.read_volatile()
}


