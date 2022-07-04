/*
   ページテーブルを作成するための機構を提供します
*/
use bootloader::boot_info::MemoryRegions;

use crate::cell::sync_once_cell::SyncOnceCell;
use crate::paging::frame_allocator::boot_info::BootInfoFrameAllocator;

pub mod boot_info;

pub(crate) static mut FRAME_ALLOCATOR: SyncOnceCell<BootInfoFrameAllocator> = SyncOnceCell::new();

pub(crate) unsafe fn init(memory_regions: &'static MemoryRegions) {
    FRAME_ALLOCATOR
        .set(BootInfoFrameAllocator::init(memory_regions))
        .expect("Failed Init Frame Allocator");
}