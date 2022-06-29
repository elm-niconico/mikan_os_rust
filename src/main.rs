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

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use mikan_os_rust::usb::pci::configuration::tmp_find_usb_mouse_base;
use mikan_os_rust::usb::xhci::controller::xhc::XhcController;
use mikan_os_rust::{println, serial_println};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    #[cfg(test)]
    test_main();

    serial_println!("Hello World! {}", 0b100000 * 1024);

    let mmio_base = tmp_find_usb_mouse_base().unwrap() + boot_info.physical_memory_offset;

    println!("mmio_base {}", mmio_base);
    println!(
        "physical_memory_offset {}",
        boot_info.physical_memory_offset
    );
    let mut xhc = XhcController::initialize(mmio_base, boot_info.physical_memory_offset, 6)
        .expect("FAILED RESULT");

    xhc.run().expect("Failed Running Xhc Controller");

    xhc.reset_all_ports().expect("Failed Reset Ports");

    let mut count = 0;

    loop {
        xhc.process_event();
    }

    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use mikan_os_rust::qemu::{exit_qemu, ExitCode};

    mikan_os_rust::println!("panic!!");
    mikan_os_rust::println!("{}", info);
    serial_println!("{}", info);
    exit_qemu(ExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[panic_handler]
// TODO Panic Handlerの定義
fn panic(info: &PanicInfo) -> ! {
    use mikan_os_rust::test_panic_handler;

    test_panic_handler(info);

    loop {
        x86_64::instructions::hlt()
    }
}