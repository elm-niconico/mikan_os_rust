use bootloader::boot_info::MemoryRegions;
use x86_64::structures::paging::OffsetPageTable;

use crate::spin::sync_once_cell::StaticOnceCell;
use crate::memory::frame::boot_info::BootInfoFrameAllocator;
use crate::memory::frame::frame_init::InitAllocator;
use crate::spin::sync_mutex::StaticSpinMutex;

mod boot_info;
mod bit_map;
mod bit_map_manager;
mod frame_id;
mod frame_init;


pub(crate) static mut FRAME_ALLOCATOR: StaticOnceCell<StaticSpinMutex<BootInfoFrameAllocator>> = StaticOnceCell::uninit();

pub(crate) unsafe fn init(memory_regions: &'static MemoryRegions) {
    FRAME_ALLOCATOR.init_once(||StaticSpinMutex::new(BootInfoFrameAllocator::new(memory_regions)))
}