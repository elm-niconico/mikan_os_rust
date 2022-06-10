#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![feature(generic_associated_types)]
#![test_runner(mikan_os_rust::test_runner_handler)]
#![reexport_test_harness_main = "test_main"]
#![feature(lang_items, alloc_error_handler)]
#![feature(once_cell)]

extern crate alloc;
extern crate bitfield_struct;

use core::fmt::{Debug, Formatter};
use core::mem;
use core::num::NonZeroUsize;
use core::panic::PanicInfo;

use bitfield_struct::bitfield;
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};
use xhci::Registers;

use crate::pci::{read_data, write_address, ConfigRegisterFinder, Device};
use crate::qemu::{exit_qemu, QemuExitCode};

mod allocator;
mod asm_func;
mod pci;
mod qemu;
mod serial_port;
mod testable;
mod usb;
mod vga_buffer;

static mut LEVEL_4_PAGE_TABLE: PageTable = PageTable::new();

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World!");

    // let xhc_dev = find_xhc_device().unwrap();

    // let base_bus = read_bar(&xhc_dev, 0).unwrap();

    // let xhc_mmio_base = to_base_bar(base_bus);

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // unsafe {
    //     LEVEL_4_PAGE_TABLE = active_level_4_table(phys_mem_offset);
    //     ALLOC = Some(BootInfoFrameAllocator::init(&boot_info.memory_map));
    // }

    // let mapper = unsafe { init(phys_mem_offset, boot_info) };
    // let mut registers = unsafe { Registers::new(xhc_mmio_base as usize, mapper) };

    // while registers.operational.usbsts.read_volatile().hc_halted() {}
    // println!(
    //     "usb command is reset! {:?}",
    //     registers.operational.usbsts.read_volatile()
    // );

    // loop {
    //     let dep = registers
    //         .interrupt_register_set
    //         .read_volatile_at(0)
    //         .erdp
    //         .event_ring_dequeue_pointer();
    //     let normal = unsafe { mem::transmute::<u64, TrbInfo>(dep) };
    //     println!("usb event {}", normal.trb_type());
    // }
    // // let mut usb_cmd = registers.operational.usbcmd;
    // // usb_cmd.update_volatile(|v| {
    // //     let mut a: u32= unsafe { mem::transmute::<UsbCommandRegister, u32>(*v) };
    // //
    // //     a.set_bit(1, true);
    // // });
    // //
    // // while !usb_cmd.read().host_controller_reset() {
    // //     let bit = unsafe { mem::transmute::<UsbCommandRegister, u32>(usb_cmd.read()) };
    // //
    // // }

    println!("usb command run_stop");
    println!("end kernel");
    #[cfg(test)]
    test_main();

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

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use mikan_os_rust::test_panic_handler;

    test_panic_handler(info);
    loop {}
}

fn find_xhc_device() -> Option<Device> {
    let mut config_register = ConfigRegisterFinder::new();
    config_register.scan_all_device().expect("failed scan");

    for i in 0..config_register.device_count {
        let dev = config_register.devices[i];
        if let Some(device) = dev {
            let is_xhci = device.class_code.is_match(0x0C, 0x03, 0x30);
            if is_xhci {
                return Some(device);
            }
        }
    }
    None
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

fn to_base_bar(bar: u64) -> u64 {
    let mask: u64 = 0xf;

    bar & !mask
}

fn make_address(bus: u32, device: u32, func: u32, reg_addr: u32) -> u32 {
    let shl = |x: u32, bits: usize| -> u32 { (x << bits) as u32 };

    let addr: u32 =
        shl(1, 31) | shl(bus, 16) | shl(device, 11) | shl(func, 8) | (reg_addr & 0xFC) as u32;
    addr as u32
}

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
// unsafe fn init(physical_memory_offset: VirtAddr, boot_info: &'static BootInfo) -> MemoryMapper {
//     let offset_page_table = OffsetPageTable::new(&mut LEVEL_4_PAGE_TABLE, physical_memory_offset);
//     MemoryMapper::new(offset_page_table, boot_info)
// }

struct MemoryMapper {
    offset_page_table: OffsetPageTable<'static>,

    boot_info: &'static BootInfo,

    page: Option<Page>,
}

impl MemoryMapper {
    pub fn new(base: OffsetPageTable<'static>, boot_info: &'static BootInfo) -> Self {
        Self {
            offset_page_table: base,
            boot_info,
            page: None,
        }
    }
}

impl xhci::accessor::Mapper for MemoryMapper {
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
        // 未使用のページをマップする
        let offset = self.boot_info.physical_memory_offset as usize;
        // use x86_64::structures::paging::PageTableFlags as Flags;
        // //let page = Page::<Size4KiB>::containing_address(VirtAddr::new((phys_start + offset) as u64));
        //
        // let frame = PhysFrame::containing_address(PhysAddr::new(phys_start as u64));
        // println!("frame {:?}", frame);
        // let page = Page::<Size4KiB>::containing_address(VirtAddr::new((phys_start + offset) as u64));
        // println!("page {:?}", page);
        // let flags = Flags::PRESENT | Flags::WRITABLE;
        // self.page = Some(page);
        //
        // let allocator= &mut ALLOC.unwrap();
        // let result = self.offset_page_table.map_to(page, frame, flags, allocator).expect("failed to map");
        // println!("result {:?}", result);
        //
        // result.flush();
        NonZeroUsize::new_unchecked((phys_start + offset))
    }

    fn unmap(&mut self, virt_start: usize, bytes: usize) {
        println!("unmap to {}", virt_start);

        let result = self
            .offset_page_table
            .unmap(self.page.expect("not page"))
            .expect("failed to unmap");
        result.1.flush();
    }
}

impl Clone for MemoryMapper {
    fn clone(&self) -> Self {
        unsafe {
            MemoryMapper::new(
                OffsetPageTable::new(
                    &mut LEVEL_4_PAGE_TABLE,
                    VirtAddr::new(self.boot_info.physical_memory_offset),
                ),
                self.boot_info,
            )
        }
    }
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
// unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> PageTable {
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

/// ブートローダのメモリマップから、使用可能な
/// フレームを返すFrameAllocator
#[derive(Copy, Clone)]
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// 渡されたメモリマップからFrameAllocatorを作る。
    ///
    /// この関数はunsafeである：呼び出し元は渡された
    /// メモリマップが有効であることを保証しなければ
    /// ならない。特に、`USABLE`なフレームは実際に
    /// 未使用でなくてはならない。
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// メモリマップによって指定されたusableなフレームのイテレータを返す。
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // メモリマップからusableな領域を得る
        let regions = self.memory_map.iter();

        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        // それぞれの領域をアドレス範囲にmapで変換する
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());

        // フレームの開始アドレスのイテレータへと変換する
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

        // 開始アドレスから`PhysFrame`型を作る
        frame_addresses.map(|addr| {
            let frame = PhysFrame::containing_address(PhysAddr::new(addr));
            frame
        })
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        println!("next alloc {}", self.next);

        let alloc = self.usable_frames().nth(self.next);
        self.next += 1;
        alloc
    }
}
