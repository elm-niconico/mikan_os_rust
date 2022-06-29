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

use alloc::rc::Rc;
use alloc::vec;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::OffsetPageTable;
use x86_64::VirtAddr;

use mikan_os_rust::allocators::init_heap;
use mikan_os_rust::page::active_level_4_table;
use mikan_os_rust::page::frame_allocator::boot_info::BootInfoFrameAllocator;
use mikan_os_rust::{println, serial_println};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    serial_println!("Hello World!");

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut level_4_table = unsafe { active_level_4_table(physical_memory_offset) };
    let mut mapper = unsafe { OffsetPageTable::new(&mut (*level_4_table), physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    unsafe {
        init_heap(&mut mapper, &mut frame_allocator).expect("Failed Init Heap");
    };

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );
    #[cfg(test)]
    test_main();

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