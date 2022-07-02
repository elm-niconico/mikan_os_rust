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

use core::alloc::GlobalAlloc;
use core::num::NonZeroUsize;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::VirtAddr;
use xhci::Registers;

use mikan_os_rust::allocators::init_heap;
use mikan_os_rust::page::active_level_4_table;
use mikan_os_rust::page::frame_allocator::boot_info::BootInfoFrameAllocator;
use mikan_os_rust::usb_my::pci::configuration::tmp_find_usb_mouse_base;
use mikan_os_rust::{interrupt, println, serial_println};

entry_point!(kernel_main);
#[derive(Copy, Clone)]
struct TestMapper {
    level_4_table: *mut PageTable,

    physical_memory_offset: VirtAddr,
}

impl xhci::accessor::Mapper for TestMapper {
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
        use x86_64::structures::paging::Mapper;
        // let mut mapper = unsafe {
        //     OffsetPageTable::new(&mut (*self.level_4_table), self.physical_memory_offset)
        // };
        // let page: Page<Size4KiB> = x86_64::structures::paging::Page::containing_address(
        //     VirtAddr::new((phys_start) as u64),
        // );
        //
        // let frame = PhysFrame::containing_address(PhysAddr::new(phys_start as u64));
        // let frags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        // let r = mapper.map_to(page, frame, frags, &mut FRAME_ALLOC.unwrap());
        // if let Ok(r) = r {
        //     r.flush();
        // }
        //let frame = FRAME_ALLOC.unwrap().allocate_frame().unwrap();
        NonZeroUsize::new_unchecked(
            (phys_start as u64 + self.physical_memory_offset.as_u64()) as usize,
        )
    }

    fn unmap(&mut self, virt_start: usize, bytes: usize) {
        todo!()
    }
}

pub fn init() {
    mikan_os_rust::gdt::init();
    interrupt::init_idt();
    unsafe { interrupt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

static mut FRAME_ALLOC: Option<BootInfoFrameAllocator> = Option::None;

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    serial_println!("Hello World!");
    println!("Hello World!");

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let level_4_table = unsafe { active_level_4_table(physical_memory_offset) };
    let mut mapper = unsafe { OffsetPageTable::new(&mut (*level_4_table), physical_memory_offset) };
    //let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    unsafe { FRAME_ALLOC = Some(BootInfoFrameAllocator::init(&boot_info.memory_map)) };

    unsafe {
        init_heap(&mut mapper, &mut FRAME_ALLOC.unwrap()).expect("Failed Init Heap");
    };

    let mut mapper = TestMapper {
        level_4_table,
        physical_memory_offset,
    };

    init();
    //let bsp_local_apic_id = 0xfee00020 >> 24;

    //let device = find_xhc_device().expect("Failed Found Device");
    // configure_msi_fixed_destination(&device, bsp_local_apic_id, true, 0, 0x40, 0);

    let xhc_mmio_base = tmp_find_usb_mouse_base().unwrap();
    // let xhc_mmio_base = xhc_mmio_base + boot_info.physical_memory_offset;

    let mut registers = unsafe { Registers::new(xhc_mmio_base as usize, mapper) };
    println!("New Registers!");
    registers.interrupt_register_set.update_volatile_at(0, |r| {
        r.imod.set_interrupt_moderation_interval(400);
        r.iman.set_interrupt_enable();
        r.iman.clear_interrupt_pending();
    });
    //
    // println!("{:?}", registers.interrupt_register_set.read_volatile_at(0));
    //let mut b = registers.interrupt_register_set.read_volatile_at(0);

    // let a = unsafe {
    //     HEAP.alloc_zeroed(Layout::from_size_align(mem::size_of::<u128>() * 1024, 64).unwrap())
    // };
    // unsafe {
    //     let mut mapper =
    //         unsafe { OffsetPageTable::new(&mut (*level_4_table), physical_memory_offset) };
    //     let page =
    //         x86_64::structures::paging::Page::containing_address(VirtAddr::new(a.addr() as u64));
    //     let frame = frame_allocator.allocate_frame().unwrap();
    //     let frags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    //     let r = mapper.map_to(page, frame, frags, &mut frame_allocator);
    //     if let Ok(r) = r {
    //         r.flush();
    //     }
    //     b.erdp
    //         .set_event_ring_dequeue_pointer(VirtAddr::new(a.addr() as u64).as_u64());
    // };
    // registers.interrupt_register_set.write_volatile_at(0, b);
    // let command_buff_ptr = unsafe {
    //     HEAP.alloc(Layout::from_size_align_unchecked(
    //         mem::size_of::<u128>() * 64,
    //         64,
    //     ))
    // };
    // let mut command_buff_array_ptr = (command_buff_ptr as *mut _) as *mut [u32; 4];
    // registers.operational.usbcmd.update_volatile(|r| {
    //     r.set_host_controller_reset();
    // });
    // while registers
    //     .operational
    //     .usbcmd
    //     .read_volatile()
    //     .host_controller_reset()
    // {}
    // while registers
    //     .operational
    //     .usbsts
    //     .read_volatile()
    //     .controller_not_ready()
    // {}
    //
    // registers.operational.crcr.update_volatile(|r| {
    //     r.set_command_ring_pointer(
    //         command_buff_array_ptr.addr() as u64 + boot_info.physical_memory_offset,
    //     )
    // });
    // registers.operational.usbcmd.update_volatile(|r| {
    //     r.set_run_stop();
    // });
    // while registers.operational.usbsts.read_volatile().hc_halted() {}
    //
    // let mut is_cycle = true;
    // let mut cahche = 0;
    // let mut count = 0;
    //
    // loop {
    //     unsafe {
    //         let mut primary_interrupt = registers.interrupt_register_set.read_volatile_at(0);
    //
    //         let mut dequeue_ptr = primary_interrupt.erdp.event_ring_dequeue_pointer();
    //         let mut dequeue_ptr = dequeue_ptr + boot_info.physical_memory_offset;
    //
    //         let mut trb_base = &mut *(dequeue_ptr as *mut TrbBase);
    //
    //         // if (trb_base.trb_type() != 0) {
    //         //     serial_println!("deque {:?}", trb_base);
    //         // }
    //         if is_cycle != trb_base.cycle_bit() {
    //             println!("SKIP {:?}", trb_base);
    //             continue;
    //         }
    //
    //         serial_println!("Normal {} {:?}", count, trb_base);
    //
    //         if trb_base.trb_type() == 32 {
    //             let normal = dequeue_ptr as *const TransferEvent;
    //             let event_value = (dequeue_ptr as *mut [u32; 4]);
    //             serial_println!("Event Buffer {:?}", *event_value);
    //             // let input = Input::new_64byte();
    //             // let data = slice::from_raw_parts_mut::<u32>(dequeue_ptr as *mut u32, 4);
    //             // let cicle = if is_cycle { 1 } else { 0 };
    //             // data[3] = (data[3] & 0xfffffffe) | cicle;
    //             // //end_point.set_input_context_pointer(input.control().)
    //             // let new = data.as_ptr() as *const ConfigureEndpoint;
    //             // let command_ring = registers.operational.crcr.read_volatile();
    //             // let data = (&command_ring) as *const _;
    //             // let data = data as *const u64;
    //             // let ptr = ((*data) << 6);
    //             //serial_println!("COMMAND RING {}", ptr);
    //             // let mut command_ring_buff = (ptr) as *mut [u32; 4];
    //
    //             let mut data = *event_value;
    //             let cicle = if is_cycle { 1 } else { 0 };
    //             data[3] = (data[3] & 0xfffffffe) | cicle;
    //             command_buff_array_ptr.write_volatile(data);
    //             serial_println!("is run {:?}", registers.operational.crcr.read_volatile());
    //             // let a = (data.as_ptr()) as *const u128;
    //             //  buffer.0[count] = data;
    //
    //             serial_println!("Command Push End");
    //             // buffer.write_volatile(ptr::read_volatile(new));
    //
    //             let mut doorbell = registers.doorbell.read_volatile_at(0);
    //
    //             doorbell.set_doorbell_target(0);
    //             doorbell.set_doorbell_stream_id(0);
    //             registers.doorbell.write_volatile_at(0, doorbell);
    //             serial_println!("END Normal");
    //         }
    //
    //         if trb_base.trb_type() == 33 {
    //             let comp_trb = dequeue_ptr as *mut xhci::ring::trb::event::CommandCompletion;
    //             serial_println!("Command Completion {:?}", *comp_trb);
    //             let ptr =
    //                 ((*comp_trb).command_trb_pointer() << 4) + boot_info.physical_memory_offset;
    //             let next_trb = ptr as *mut TrbBase;
    //             serial_println!("Issure Type {:?}", *next_trb)
    //         }
    //
    //         registers
    //             .interrupt_register_set
    //             .update_volatile_at(0, |primary| {
    //                 let mut erdp = primary.erdp.event_ring_dequeue_pointer();
    //                 let end_ptr = primary.erstba.get() + primary.erstsz.get() as u64;
    //                 let next_ptr = erdp + 16;
    //                 serial_println!("end {} next {}", end_ptr, next_ptr);
    //                 let next_ptr = if next_ptr >= end_ptr {
    //                     is_cycle = !is_cycle;
    //                     primary.erstba.get()
    //                 } else {
    //                     next_ptr
    //                 };
    //
    //                 serial_println!("dequeue {}", next_ptr);
    //                 primary.erdp.set_event_ring_dequeue_pointer(next_ptr);
    //             });
    // println!("status {}", base.status());
    // let data = slice::from_raw_parts::<u32>(dequeue_ptr as *const _, 4);
    // let data = unsafe { <[u32; 4]>::try_from(data).unwrap() };
    // let trb = TransferEvent::try_from(data);
    //     }
    // }
    #[cfg(test)]
    test_main();

    loop {
        x86_64::instructions::hlt();
    }
}

fn init_allocator(boot_info: &'static BootInfo) {
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let level_4_table = unsafe { active_level_4_table(physical_memory_offset) };
    let mut mapper = unsafe { OffsetPageTable::new(&mut (*level_4_table), physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    unsafe {
        init_heap(&mut mapper, &mut frame_allocator).expect("Failed Init Heap");
    };
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