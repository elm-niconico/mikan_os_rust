use bootloader::boot_info::MemoryRegions;

use crate::cell::sync_once_cell::SyncOnceCell;
use crate::memory::frame::boot_info::BootInfoFrameAllocator;
use crate::memory::frame::frame_init::InitAllocator;

mod boot_info;
mod bit_map;
mod bit_map_manager;
mod frame_id;
mod frame_init;


pub(crate) static mut FRAME_ALLOCATOR: SyncOnceCell<BootInfoFrameAllocator> = SyncOnceCell::<BootInfoFrameAllocator>::new();

pub(crate) unsafe fn init(memory_regions: &'static MemoryRegions) {
    FRAME_ALLOCATOR
        .set(BootInfoFrameAllocator::new(memory_regions));
}