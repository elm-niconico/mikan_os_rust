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

// extern crate alloc;
extern crate bitfield_struct;
// extern crate rlibc;


use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

use crate::frame_buffer::pixel::pixel_color::PixelColor;
use crate::frame_buffer::pixel::pixel_writer::{PixelWriter, Rect};
use crate::frame_buffer::WRITER;
use crate::PixelColor::Rgb;
use crate::qemu::{exit_qemu, ExitCode};
use crate::segmentation::gdt;
use crate::testable::Testable;

mod assembly;
mod spin;
mod error;
mod frame_buffer;
// mod interrupt;
mod macros;
// mod memory;
mod qemu;
mod segmentation;
mod serial_port;
mod testable;
mod usb;
mod utils;
mod log;
mod cxx_support;

entry_point!(kernel_main);


#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // let offset = boot_info.physical_memory_offset.as_ref().copied().unwrap();
    unsafe { init_kernel(boot_info) };


    #[cfg(test)]
    test_main();


    #[allow(unreachable_code)]
    assembly::hlt::hlt_loop()
}

unsafe fn init_kernel(boot_info: &'static mut BootInfo) {
    frame_buffer::init(boot_info.framebuffer.as_mut().unwrap());
    println!("Init Frame Buffer!");

    serial_port::init();
    serial_println!("Init Serial Port!");

    // TODO スタックの移動


    draw_window();
    serial_println!("Drew Window Object");

    gdt::init();
    serial_println!("Init GDT");

    // memory::init(&boot_info.memory_regions, phys_offset_addr);
    // serial_println!("Init Memory");
    //

    //
    // interrupt::init();
    // serial_println!("Init Interrupt");
    //
    //
    // x86_64::instructions::interrupts::enable();
    // serial_println!("Interrupt Enable");
}

fn draw_window() {
    let frame_writer = &mut *WRITER.get().lock();
    let frame_width = frame_writer.width();
    let frame_height = frame_writer.height();

    // Window Background
    frame_writer.fill_rect(
        Rect::new(0, 0, frame_writer.width(), frame_writer.height()),
        PixelColor::Rgb([45, 118, 237, 0]),
    );

    // Window Bottom
    frame_writer.fill_rect(
        Rect::new(0, frame_height - 50, frame_width, 50),
        Rgb([1, 8, 17, 0]),
    );

    // Window Left Bottom Region
    frame_writer.fill_rect(
        Rect::new(0, frame_height - 50, frame_width / 5, 50),
        Rgb([80, 80, 80, 0]),
    );

    // Window Left Bottom Icon
    frame_writer.draw_rect(
        Rect::new(10, frame_height - 40, 30, 30),
        Rgb([160, 160, 160, 0]),
    )
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
    assembly::hlt::hlt_loop()
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