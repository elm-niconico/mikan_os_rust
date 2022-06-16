#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner_handler)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use core::{mem, ptr};

use mikan_os_rust::usb::pci::configuration::tmp_find_usb_mouse_base;

use mikan_os_rust::usb::xhci::registers::capability::capability_register::CapabilityRegister;
use mikan_os_rust::usb::xhci::registers::creator::operational_register_create::{
    CreateOperationalRegisters, ICreateOperationalRegisters,
};
use mikan_os_rust::usb::xhci::registers::operators::create::register_creator::{
    CapabilityRegisterCreate, ICapabilityRegisterCreate,
};

use mikan_os_rust::test_runner_handler;
use mikan_os_rust::usb::xhci::registers::read_write::volatile::IVolatile;
use mikan_os_rust::usb::xhci::registers::register_info::RegisterInfo;
use mikan_os_rust::usb::xhci::trb::transfer_request_block::TrbBase;
use mikan_os_rust::{impl_deref_from_type, serial_println};

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
pub fn should_debug_trb_base() {
    let trb_base = TrbBase::test_new();

    assert_eq!(1, 1);
}

#[test_case]
pub fn shold_impl_deref() {
    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
    struct A(u32);
    impl_deref_from_type!(A, u32);

    let a = A(32);
    serial_println!("{:?}", *a);
    assert_eq!(32, 32);
}

// #[test_case]
// pub fn should_alloc_capability() {
//     let mmio_base = tmp_find_usb_mouse_base().unwrap();
//     let create = CapabilityRegisterCreate::Transmute;
//     let capability_register = create.create(mmio_base + OFFSET);

//     assert!(capability_register.is_ok());
//     let volatile = capability_register.unwrap();
//     let register_info = volatile.read_volatile();
//     serial_println!("{:?}", register_info);
// }

#[test_case]
pub fn should_update_register() {

    // let register = extract_cap_register(extract_virtual_mmio_base_addr())
    //
    // let register = volatile.read_volatile();
    // let cap_len = register.cap_length;
    // volatile.update_volatile(|r| {
    //     r.hcc_params1.set_number_of_device_slots(100);
    // });
    //
    // let cap_register = volatile.read_volatile();
    //
    // assert!(true);
    // //todo!("後程実装");
    //assert_eq!(100, cap_register.hcc_params1.number_of_device_slots());
}

#[test_case]
pub fn should_cast_to_operational_registers() {
    let mmio_base = extract_virtual_mmio_base_addr();

    let cap_len: u8 = extract_cap_register(mmio_base).cap_length.into();

    let registers = CreateOperationalRegisters::UncheckTransmute.create(mmio_base, cap_len);

    assert!(registers.is_ok());
    serial_println!("Operational Registers {:?}", registers.unwrap());
}

fn extract_virtual_mmio_base_addr() -> u64 {
    let mmio_base = tmp_find_usb_mouse_base().unwrap();
    mmio_base + OFFSET
}

fn extract_cap_register(mmio_base: u64) -> CapabilityRegister {
    let create = CapabilityRegisterCreate::Transmute;
    let volatile = create
        .create(mmio_base)
        .expect("Failed Mapping to Cap Register");

    volatile.read_volatile()
}

fn transmute<T: core::fmt::Debug>(addr: &mut u64) -> RegisterInfo<T> {
    let ptr = *addr as *const T;
    let size = mem::size_of::<T>();

    let register = unsafe { ptr::read_volatile(ptr) };
    let info = RegisterInfo::new(addr.clone(), register);
    *addr += size as u64;
    info
}
