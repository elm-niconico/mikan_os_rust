#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(strict_provenance)]
#![test_runner(crate::test_runner_handler)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

use crate::qemu::{exit_qemu, QemuExitCode};
use crate::testable::Testable;


mod asm_func;

mod macros;
pub mod qemu;
pub mod serial_port;
pub mod testable;
pub mod usb;
pub mod vga_buffer;
mod utils;

#[cfg(test)]
entry_point!(test_kernel_main);

pub fn test_runner_handler(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    
    exit_qemu(QemuExitCode::Success);
}


pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    test_main();
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
