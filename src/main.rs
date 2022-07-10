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
#![feature(portable_simd)]
#![feature(abi_x86_interrupt)]

extern crate alloc;
extern crate bitfield_struct;
// extern crate rlibc;


use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

use segmentation::gdt;

use crate::interrupt::apic::mouse::XHC_MOUSE;
use crate::interrupt::map;
use crate::memory::frame::FRAME_ALLOCATOR;
use crate::memory::paging::PAGE_TABLE;
use crate::qemu::{exit_qemu, ExitCode};
use crate::testable::Testable;
use crate::usb::pci::configuration::tmp_find_usb_mouse_base;

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


entry_point!(kernel_main);


#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    unsafe { init_kernel(boot_info) };


    #[cfg(test)]
    test_main();

    let mut mouse = unsafe { XHC_MOUSE.get_unchecked().lock() };
    mouse.run();

    mouse.ports();
    loop {
        mouse.process_event();
    }

    #[allow(unreachable_code)]
    assembly::hlt_loop()
}

unsafe fn init_kernel(boot_info: &'static mut BootInfo) {
    let phys_offset_addr = VirtAddr::new(boot_info.physical_memory_offset.as_ref().copied().unwrap());

    frame_buffer::init(boot_info.framebuffer.as_mut().unwrap());
    serial_port::init();

    serial_println!("Init Frame Buffer");

    serial_println!("Offset {:?}", phys_offset_addr.as_u64());


    memory::init(&boot_info.memory_regions, phys_offset_addr);
    serial_println!("Init Memory");

    gdt::init();
    serial_println!("Init GDT");


    interrupt::init(phys_offset_addr);
    serial_println!("Init Interrupt");


    x86_64::instructions::interrupts::enable();
    serial_println!("Interrupt Enable");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("Panic!!");
    serial_println!("{:?}", info);
    qemu::exit_qemu(qemu::ExitCode::Failed);
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

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}