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

use bootloader::boot_info::Optional;
use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::{Mapper, Translate};
use x86_64::VirtAddr;

use crate::paging::PAGE_MAPPER;
use crate::qemu::{exit_qemu, ExitCode};
use crate::testable::Testable;

mod allocators;
mod asm_func;
mod cell;
mod error;
mod frame_buffer;
mod gdt;
mod interrupt;
mod macros;
mod paging;
mod qemu;
mod serial_port;
mod testable;
mod usb;
mod utils;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let physical_offset = offset_as_u64(boot_info.physical_memory_offset);

    unsafe { init_kernel(VirtAddr::new(physical_offset), boot_info) };
    log!("hello world!");

    // let mapper = unsafe { init_mapper(VirtAddr::new(physical_offset)) };
    // let xhc_mmio_base = tmp_find_usb_mouse_base().unwrap();
    // println!(
    //     "Xhc Mmio Base physical {:?} {}",
    //     mapper
    //         .translate_addr(VirtAddr::new(xhc_mmio_base + physical_offset))
    //         .unwrap()
    //         .as_u64(),
    //     VirtAddr::new(xhc_mmio_base).as_u64()
    // );

    // print_virtual_addr!(physical_offset);
    //
    // print_virtual_addr!("stack", 0x1100_1020_1a10);
    //let virtual_offset = VirtAddr::new(physical_offset);

    // let level_4_table = unsafe { active_level_4_table(physical_offset) };
    // unsafe { print_all_use_entries(&*level_4_table) };

    // serial_println!("Hello World!");

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

    #[cfg(test)]
    test_main();
    // x86_64::instructions::interrupts::int3();
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

unsafe fn init_kernel(physical_memory_offset: VirtAddr, boot_info: &'static mut BootInfo) {
    let addr = {
        boot_info
            .framebuffer
            .as_ref()
            .unwrap()
            .buffer()
            .as_ptr()
            .addr()
    };
    frame_buffer::init(boot_info.framebuffer.as_mut().unwrap());
    paging::init(physical_memory_offset, &mut boot_info.memory_regions);

    let frame_buff_physical_addr = PAGE_MAPPER
        .get_mut()
        .unwrap()
        .translate_addr(VirtAddr::new(addr as u64));

    log!(
        "Frame Buffer Physical Addr {:?} Physical memory Offset Virtual Address {} FrameBuffer Virtual Address {}",
        frame_buff_physical_addr.unwrap().as_u64(),
        physical_memory_offset.as_u64(),
        VirtAddr::new(addr as u64).as_u64()
    );

    gdt::init();
    interrupt::init_idt(physical_memory_offset).expect("Failed Init Interrupter");
    interrupt::PICS.lock().initialize();
    x86_64::instructions::interrupts::enable();
}

fn offset_as_u64(physical_memory_offset: Optional<u64>) -> u64 {
    physical_memory_offset.as_ref().copied().unwrap()
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