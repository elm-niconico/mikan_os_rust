use bootloader::boot_info::MemoryRegions;
use x86_64::VirtAddr;

use crate::{FRAME_ALLOCATOR, PAGE_TABLE, serial_println};

pub mod paging;
pub mod heap;
pub mod frame;


pub unsafe fn init(memory_regions: &'static MemoryRegions, phys_addr: VirtAddr) {
    frame::init(memory_regions);
    serial_println!("Init Frame Allocator");


    paging::init(phys_addr);
    let offset_table = &mut *PAGE_TABLE.get_unchecked().lock();
    heap::init_heap(offset_table, &mut *FRAME_ALLOCATOR.lock()).expect("Failed To Init Heap");
}