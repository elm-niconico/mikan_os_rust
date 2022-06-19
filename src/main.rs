#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![feature(generic_associated_types)]
#![test_runner(mikan_os_rust::test_runner_handler)]
#![reexport_test_harness_main = "test_main"]
#![feature(lang_items, alloc_error_handler)]
#![feature(once_cell)]
#![feature(strict_provenance)]
extern crate alloc;
extern crate bitfield_struct;


use core::fmt::{Debug, Formatter};
use core::panic::PanicInfo;

use bitfield_struct::bitfield;
use bootloader::{BootInfo, entry_point};

use mikan_os_rust::{println, serial_println};
use mikan_os_rust::qemu::{exit_qemu, ExitCode};
use mikan_os_rust::usb::pci::configuration::tmp_find_usb_mouse_base;
use mikan_os_rust::usb::xhci::controller::xhc::XhcController;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    serial_println!("Hello World! {}", boot_info.physical_memory_offset);
    #[cfg(test)]
    test_main();
    
    
    let mmio_base_addr = tmp_find_usb_mouse_base().unwrap() + boot_info.physical_memory_offset;
    let xhc_controller = XhcController::initialize(mmio_base_addr, 8).expect("Failed Create Contorller");
    
    
    loop {}
}


#[bitfield(u64)]
struct TrbInfo {
    parameter: u16,
    status: u16,
    #[bits(1)]
    pub cycle_bit: u8,
    #[bits(1)]
    pub evaluate_next_trb: usize,
    #[bits(8)]
    _pad: usize,
    #[bits(6)]
    pub trb_type: usize,
    #[bits(16)]
    pub control: usize,
}


impl Debug for TrbInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "\
        parameter {}\n\
        status {}\n\
        trb_type {}
        ",
            self.parameter(),
            self.status(),
            self.trb_type()
        ))
    }
}


#[bitfield(u64)]
struct PageTableEntry {
    /// defaults to 32 bits for u32
    addr: u32,
    
    /// public field -> public accessor functions
    #[bits(12)]
    pub size: usize,
    
    /// padding: No accessor functions are generated for fields beginning with `_`.
    #[bits(6)]
    _p: u8,
    
    /// interpreted as 1 bit flag
    present: bool,
    
    /// sign extend for signed integers
    #[bits(13)]
    negative: i16,
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panic!!");
    println!("{}", info);
    serial_println!("{}", info);
    exit_qemu(ExitCode::Failed);
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use mikan_os_rust::test_panic_handler;
    
    test_panic_handler(info);
    loop {}
}

