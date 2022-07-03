use bootloader::boot_info::MemoryRegions;
use x86_64::VirtAddr;

mod frame_allocator;
mod page_mapper;

pub(crate) use frame_allocator::FRAME_ALLOCATOR;
pub(crate) use page_mapper::PAGE_MAPPER;

pub(crate) unsafe fn init(
    physical_memory_offset: VirtAddr,
    memory_regions: &'static mut MemoryRegions,
) {
    frame_allocator::init(memory_regions);
    page_mapper::init(physical_memory_offset);
}