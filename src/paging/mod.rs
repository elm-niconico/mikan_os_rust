use crate::paging::frame_allocator::boot_info::BootInfoFrameAllocator;
use crate::paging::frame_allocator::FRAME_ALLOCATOR;
use bootloader::boot_info::MemoryRegions;

pub mod frame_allocator;
pub mod page_table;

pub(crate) unsafe fn init(memory_regions: &'static mut MemoryRegions) {
    FRAME_ALLOCATOR
        .set(BootInfoFrameAllocator::init(memory_regions))
        .expect("Failed Init Frame Allocator");
}