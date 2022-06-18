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


use core::fmt::{Debug, Formatter};
use core::panic::PanicInfo;

use bitfield_struct::bitfield;
use bootloader::{BootInfo, entry_point};

use mikan_os_rust::usb::xhci::controller::xhc::XhcController;

use crate::qemu::exit_qemu;
use crate::QemuExitCode::Failed;
use crate::usb::pci::configuration::{Device, read_data, tmp_find_usb_mouse_base, write_address};


mod asm_func;
mod macros;
mod qemu;
mod serial_port;
mod testable;
mod usb;
mod vga_buffer;
mod utils;
mod allocators;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    serial_println!("Hello World! {}", boot_info.physical_memory_offset);
    #[cfg(test)]
    test_main();
    let xhc_mmio_base = tmp_find_usb_mouse_base().unwrap() + boot_info.physical_memory_offset;
    let xhc_controller = XhcController::new(xhc_mmio_base).expect("Failed Create Contorller");
    println!("{:?}", xhc_controller);
    
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    //
    // serial_println!("offset {}", boot_info.physical_memory_offset);
    // let mapper = utils { init(phys_mem_offset, boot_info) };
    //
    // let xhc_mmio_base = tmp_find_usb_mouse_base().unwrap();
    // let range = 0..7;
    // let cap = xhc_mmio_base.get_bits(range) as u8;
    // serial_println!("cap_length {:?}", cap);
    //
    // let mut registers = utils { Registers::new(xhc_mmio_base as usize, mapper) };
    // registers.interrupt_register_set.update_volatile_at(0, |r| {
    //     r.imod.set_interrupt_moderation_interval(4000);
    // });
    // registers.interrupt_register_set.update_volatile_at(0, |r| {
    //     r.iman.set_interrupt_enable();
    //     r.iman.clear_interrupt_pending();
    // });
    // registers.operational.usbcmd.update_volatile(|r| {
    //     r.set_interrupter_enable();
    //     r.set_run_stop();
    // });
    //
    // while registers.operational.usbsts.read_volatile().hc_halted() {}
    //
    // println!(
    //     "imod {:?}",
    //     registers.interrupt_register_set.read_volatile_at(0).iman
    // );
    //
    // let trb_ptr = (dequeue_ptr + boot_info.physical_memory_offset) as *const _;
    //
    // let trb_data = utils { slice::from_raw_parts::<u32>(trb_ptr, 4) };
    // let trb_data = utils { <[u32; 4]>::try_from(trb_data).unwrap() };
    //
    
    //let trb = TrbBase::from(dequeue_ptr);
    
    //println!("trb array {:?}", trb_pointer);
    //
    // let mut event_count = 0;
    // let mut is_cycle = true;
    // let mut cahche = 0;
    // serial_println!(
    //     "event {:?}",
    //     registers.port_register_set.read_volatile_at(0).portsc
    // );
    // loop {
    //     utils {
    //         let mut primary_interrupt = registers.interrupt_register_set.read_volatile_at(0);
    //
    //         let mut dequeue_ptr = primary_interrupt.erdp.event_ring_dequeue_pointer();
    //         let mut dequeue_ptr = dequeue_ptr + boot_info.physical_memory_offset;
    //         if cahche != dequeue_ptr {
    //             cahche = dequeue_ptr;
    //             serial_println!("cache {}", dequeue_ptr);
    //         }
    //         let mut trb_base = &mut *(dequeue_ptr as *mut TrbBase);
    //
    //         //
    //         // if (trb_base.trb_type() != 0) {
    //         //     serial_println!("deque {:?}", trb_base);
    //         // }
    //         if is_cycle != trb_base.cycle_bit() {
    //             continue;
    //         }
    //
    //         //println!("status {}", base.status());
    //         let data = slice::from_raw_parts::<u32>(dequeue_ptr as *const _, 4);
    //         let data = utils { <[u32; 4]>::try_from(data).unwrap() };
    //         let trb = TransferEvent::try_from(data);
    //
    //         if let Ok(trb) = trb {
    //             let slot_id = trb.slot_id();
    //
    //             if trb.completion_code().map(|c| c == Success).is_err() {
    //                 continue;
    //             }
    //             let base =
    //                 &*((trb.trb_pointer() + boot_info.physical_memory_offset) as *const TrbBase);
    //
    //             if base.trb_type() == 1 {
    //                 let normal_ptr =
    //                     (trb.trb_pointer() + boot_info.physical_memory_offset) as *const NormalTrb;
    //                 let normal = utils { ptr::read_volatile(normal_ptr) };
    //
    //                 if trb.endpoint_id() & 1 == 1 {
    //                     let buf = normal.data_buffer_pointer() + boot_info.physical_memory_offset;
    //
    //                     let buf = slice::from_raw_parts::<u8>(
    //                         buf as *const _,
    //                         normal.trb_transfer_length() as usize,
    //                     );
    //
    //                     println!("Buffer from Normal x {} y{}", buf[1], buf[2]);
    //                 }
    //             }
    //         }
    //
    //         let deqeue_ptr = primary_interrupt.erdp.event_ring_dequeue_pointer()
    //             + boot_info.physical_memory_offset;
    //         let ptr = deqeue_ptr as *const TrbBase;
    //         let ptr = ptr.add(1);
    //         let next_ptr = ptr.addr() as u64;
    //
    //         println!(
    //             "imo {:?}",
    //             registers.interrupt_register_set.read_volatile_at(0).imod
    //         );
    //         println!("usb cmd {:?}", registers.operational.usbcmd.read_volatile());
    //         registers.interrupt_register_set.update_volatile_at(0, |r| {
    //             let event_base_addr =
    //                 primary_interrupt.erstba.get() + boot_info.physical_memory_offset;
    //             let event_end_addr = event_base_addr + primary_interrupt.erstsz.get() as u64;
    //             let next_ptr =
    //                 r.erdp.event_ring_dequeue_pointer() + boot_info.physical_memory_offset + 16;
    //             if next_ptr >= event_end_addr {
    //                 r.erdp.set_event_ring_dequeue_pointer(
    //                     event_base_addr - boot_info.physical_memory_offset,
    //                 );
    //                 is_cycle = !is_cycle;
    //             } else {
    //                 r.erdp.set_event_ring_dequeue_pointer(
    //                     next_ptr - boot_info.physical_memory_offset,
    //                 );
    //             }
    //         });
    //     }
    // }
    // println!("trb {:?}", event.expect("faield result trb"));
    // loop {
    //
    //     serial_println!("base {:?}", registers.port_register_set.read_volatile_at(0).portsc);
    // }
    //
    // let base = registers.operational.dcbaap.read_volatile().get();
    // println!("usb command register {:?}", registers.operational.operators.read_volatile());
    //
    //
    // println!("len {:?}", primary_interrupt.erdp.event_ring_dequeue_pointer());
    //
    
    // loop {
    //     let dep = registers
    //         .interrupt_register_set
    //         .read_volatile_at(0)
    //         .erdp
    //         .event_ring_dequeue_pointer();
    //     let normal = utils { mem::transmute::<u64, TrbInfo>(dep) };
    //     println!("usb event {}", normal.trb_type());
    // }
    // let mut usb_cmd = registers.operational.operators;
    // usb_cmd.update_volatile(|v| {
    //     let mut a: u32= utils { mem::transmute::<UsbCommandRegister, u32>(*v) };
    //
    //     a.set_bit(1, true);
    // });
    //
    
    println!("usb command run_stop");
    println!("end kernel");
    
    loop {}
}


#[bitfield(u64)]
struct TrbInfo {
    parameter: u16,
    status: u16,
    #[bits(1)]
    pub cycle_bit: u8,
    #[bits(1)]
    pub evaluate_next_trb: usize,
    #[bits(8)]
    _pad: usize,
    #[bits(6)]
    pub trb_type: usize,
    #[bits(16)]
    pub control: usize,
}


impl Debug for TrbInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "\
        parameter {}\n\
        status {}\n\
        trb_type {}
        ",
            self.parameter(),
            self.status(),
            self.trb_type()
        ))
    }
}


#[bitfield(u64)]
struct PageTableEntry {
    /// defaults to 32 bits for u32
    addr: u32,
    
    /// public field -> public accessor functions
    #[bits(12)]
    pub size: usize,
    
    /// padding: No accessor functions are generated for fields beginning with `_`.
    #[bits(6)]
    _p: u8,
    
    /// interpreted as 1 bit flag
    present: bool,
    
    /// sign extend for signed integers
    #[bits(13)]
    negative: i16,
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panic!!");
    println!("{}", info);
    serial_println!("{}", info);
    exit_qemu(Failed);
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use mikan_os_rust::test_panic_handler;
    
    test_panic_handler(info);
    loop {}
}


pub fn read_bar(device: &Device, index: u32) -> Result<u64, ()> {
    let addr = 0x10 + 4 * index;
    let bar = read_cong(device, addr);
    
    // 32bit
    if (bar & 0x4) == 0 {
        return Ok(bar as u64);
    }
    
    let upper_bar: u64 = read_cong(device, addr + 4) as u64;
    let result: u64 = (upper_bar) << 32 | bar as u64;
    Ok(result)
}


pub fn read_cong(device: &Device, addr: u32) -> u32 {
    write_address(make_address(device.bus, device.device, device.func, addr));
    read_data()
}


fn make_address(bus: u32, device: u32, func: u32, reg_addr: u32) -> u32 {
    let shl = |x: u32, bits: usize| -> u32 { (x << bits) as u32 };
    
    let addr: u32 =
        shl(1, 31) | shl(bus, 16) | shl(device, 11) | shl(func, 8) | (reg_addr & 0xFC) as u32;
    addr as u32
}

//
// /// Initialize a new OffsetPageTable.
// ///
// /// This function is utils because the caller must guarantee that the
// /// complete physical memory is mapped to virtual memory at the passed
// /// `physical_memory_offset`. Also, this function must be only called once
// /// to avoid aliasing `&mut` references (which is undefined behavior).
// utils fn init(physical_memory_offset: VirtAddr, boot_info: &'static BootInfo) -> MemoryMapper {
//     let offset_page_table = OffsetPageTable::new(&mut LEVEL_4_PAGE_TABLE, physical_memory_offset);
//     MemoryMapper::new(offset_page_table, boot_info)
// }
//
//
// struct MemoryMapper {
//     offset_page_table: OffsetPageTable<'static>,
//
//     boot_info: &'static BootInfo,
//
//     page: Option<Page>,
// }
//
//
// impl MemoryMapper {
//     pub fn new(base: OffsetPageTable<'static>, boot_info: &'static BootInfo) -> Self {
//         Self {
//             offset_page_table: base,
//             boot_info,
//             page: None,
//         }
//     }
// }
//
//
// impl xhci::accessor::Mapper for MemoryMapper {
//     utils fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
//         // 未使用のページをマップする
//         let offset = self.boot_info.physical_memory_offset as usize;
//         // use x86_64::structures::paging::PageTableFlags as Flags;
//         // //let page = Page::<Size4KiB>::containing_address(VirtAddr::new((phys_start + offset) as u64));
//         //
//         // let frame = PhysFrame::containing_address(PhysAddr::new(phys_start as u64));
//         // println!("frame {:?}", frame);
//         // let page = Page::<Size4KiB>::containing_address(VirtAddr::new((phys_start + offset) as u64));
//         // println!("page {:?}", page);
//         // let flags = Flags::PRESENT | Flags::WRITABLE;
//         // self.page = Some(page);
//         //
//         // let allocators= &mut ALLOC.unwrap();
//         // let result = self.offset_page_table.map_to(page, frame, flags, allocators).expect("failed to map");
//         // println!("result {:?}", result);
//         //
//         // result.flush();
//         NonZeroUsize::new_unchecked((phys_start + offset))
//     }
//
//     fn unmap(&mut self, virt_start: usize, bytes: usize) {
//         println!("unmap to {}", virt_start);
//
//         let result = self
//             .offset_page_table
//             .unmap(self.page.expect("not page"))
//             .expect("failed to unmap");
//         result.1.flush();
//     }
// }
//
//
// impl Clone for MemoryMapper {
//     fn clone(&self) -> Self {
//         utils {
//             MemoryMapper::new(
//                 OffsetPageTable::new(
//                     &mut LEVEL_4_PAGE_TABLE,
//                     VirtAddr::new(self.boot_info.physical_memory_offset),
//                 ),
//                 self.boot_info,
//             )
//         }
//     }
// }

// utils fn active_level_4_table(physical_memory_offset: VirtAddr) -> PageTable {
//     use x86_64::registers::control::Cr3;
//
//     let (level_4_table_frame, _) = Cr3::read();
//
//     let phys = level_4_table_frame.start_address();
//     let virt = physical_memory_offset + phys.as_u64();
//     let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
//
//     page_table_ptr.read_volatile()
// }
//

// /// ブートローダのメモリマップから、使用可能な
// /// フレームを返すFrameAllocator
// #[derive(Copy, Clone)]
// pub struct BootInfoFrameAllocator {
//     memory_map: &'static MemoryMap,
//     next: usize,
// }
//
//
// impl BootInfoFrameAllocator {
//     /// 渡されたメモリマップからFrameAllocatorを作る。
//     ///
//     /// この関数はunsafeである：呼び出し元は渡された
//     /// メモリマップが有効であることを保証しなければ
//     /// ならない。特に、`USABLE`なフレームは実際に
//     /// 未使用でなくてはならない。
//     pub utils fn init(memory_map: &'static MemoryMap) -> Self {
//         BootInfoFrameAllocator {
//             memory_map,
//             next: 0,
//         }
//     }
//
//     /// メモリマップによって指定されたusableなフレームのイテレータを返す。
//     fn usable_frames(&self) -> impl Iterator<Item=PhysFrame> {
//         // メモリマップからusableな領域を得る
//         let regions = self.memory_map.iter();
//
//         let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
//
//         // それぞれの領域をアドレス範囲にmapで変換する
//         let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
//
//         // フレームの開始アドレスのイテレータへと変換する
//         let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
//
//         // 開始アドレスから`PhysFrame`型を作る
//         frame_addresses.map(|addr| {
//             let frame = PhysFrame::containing_address(PhysAddr::new(addr));
//             frame
//         })
//     }
// }
//
//
// utils impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
//     fn allocate_frame(&mut self) -> Option<PhysFrame> {
//         println!("next alloc {}", self.next);
//
//         let alloc = self.usable_frames().nth(self.next);
//         self.next += 1;
//         alloc
//     }
// }
