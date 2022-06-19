#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(strict_provenance)]
#![test_runner(crate::test_runner_handler)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]
#![feature(portable_simd)]


extern crate alloc;


use core::alloc::Layout;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

use crate::qemu::{exit_qemu, ExitCode};
use crate::testable::Testable;


pub mod asm_func;
pub mod macros;
pub mod qemu;
pub mod serial_port;
pub mod testable;
pub mod usb;
pub mod vga_buffer;
pub mod utils;
pub mod allocators;

#[cfg(test)]
entry_point!(test_kernel_main);

pub fn test_runner_handler(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    
    exit_qemu(ExitCode::Success);
}


pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(ExitCode::Failed);
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


#[alloc_error_handler]
pub fn on_oom(_layout: Layout) -> ! {
    println!("alloc error!");
    exit_qemu(ExitCode::Failed);
    loop {}
}
