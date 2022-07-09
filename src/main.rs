#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![feature(generic_associated_types)]
#![test_runner(test_runner_handler)]
#![reexport_test_harness_main = "test_main"]
#![feature(lang_items, alloc_error_handler)]
#![feature(once_cell)]
#![feature(default_alloc_error_handler)]
#![feature(strict_provenance)]
#![cfg_attr(test, no_main)]
#![feature(portable_simd)]
#![feature(abi_x86_interrupt)]

extern crate alloc;
extern crate bitfield_struct;

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use lazy_static::initialize;
use pic8259::ChainedPics;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{Mapper, Translate};
use x86_64::structures::paging::FrameAllocator;
use x86_64::VirtAddr;

use segmentation::gdt;

use crate::memory::frame::FRAME_ALLOCATOR;
use crate::memory::paging::PAGE_TABLE;
use crate::qemu::{exit_qemu, ExitCode};
use crate::testable::Testable;

mod assembly;
mod spin;
mod error;
mod frame_buffer;
mod interrupt;
mod macros;
mod memory;
mod qemu;
mod segmentation;
mod serial_port;
mod testable;
mod usb;
mod utils;

mod task;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    unsafe { init_kernel(boot_info) };

    #[cfg(test)]
    test_main();

    assembly::hlt_loop();
}

unsafe fn init_kernel(boot_info: &'static mut BootInfo) {
    let phys_addr = VirtAddr::new(boot_info.physical_memory_offset.as_ref().copied().unwrap());

    frame_buffer::init(boot_info.framebuffer.as_mut().unwrap());
    log!("Init Frame Buffer");

    memory::init(&boot_info.memory_regions, phys_addr);
    log!("Init Memory");

    gdt::init();
    log!("Init GDT");

    interrupt::init();
    log!("Init IDT");

    x86_64::instructions::interrupts::enable();
    log!("Interrupt Enable");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use qemu::{exit_qemu, ExitCode};

    println!("panic!!");
    println!("{}", info);
    serial_println!("{}", info);
    exit_qemu(ExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[panic_handler]
// TODO Panic Handlerの定義
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);

    #[allow(unreachable_code)]
    assembly::hlt_loop()
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

async fn async_number() -> u32 {
    42
}

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