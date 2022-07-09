use bootloader::boot_info::MemoryRegions;
use bootloader::BootInfo;
use x86_64::VirtAddr;

use crate::{FRAME_ALLOCATOR, memory, PAGE_TABLE};

pub mod paging;
pub mod heap;
pub mod frame;


pub unsafe fn init(memory_regions: &'static MemoryRegions, phys_addr: VirtAddr) {
    frame::init(memory_regions);
    paging::init(phys_addr);

    heap::init_heap(&mut *PAGE_TABLE.get().lock(), &mut *FRAME_ALLOCATOR.get().lock()).expect("Failed To Init Heap");
}