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

use bootloader::{entry_point, BootInfo};

use crate::qemu::{exit_qemu, ExitCode};
use crate::testable::Testable;

mod allocators;
mod asm_func;
mod gdt;
mod interrupt;
mod macros;
mod paging;
mod qemu;
mod serial_port;
mod testable;
mod usb;
mod utils;
mod vga_buffer;

entry_point!(kernel_main);

pub fn init() {
    gdt::init();
    interrupt::init_idt();
    unsafe { interrupt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    serial_println!("Hello World!");
    println!("Hello World!");
    //
    // let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let level_4_table = unsafe { active_level_4_table(physical_memory_offset) };
    // let mut mapper = unsafe { OffsetPageTable::new(&mut (*level_4_table), physical_memory_offset) };
    // //let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    // unsafe { FRAME_ALLOC = Some(BootInfoFrameAllocator::init(&boot_info.memory_map)) };
    //
    // unsafe {
    //     init_heap(&mut mapper, &mut FRAME_ALLOC.unwrap()).expect("Failed Init Heap");
    // };

    init();

    #[cfg(test)]
    test_main();

    loop {
        x86_64::instructions::hlt();
    }
}

// fn init_allocator(boot_info: &'static BootInfo) {
//     let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
//     let level_4_table = unsafe { active_level_4_table(physical_memory_offset) };
//     let mut mapper = unsafe { OffsetPageTable::new(&mut (*level_4_table), physical_memory_offset) };
//     let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
//
//     unsafe {
//         init_heap(&mut mapper, &mut frame_allocator).expect("Failed Init Heap");
//     };
// }

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

    loop {
        x86_64::instructions::hlt()
    }
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