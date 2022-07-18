use bootloader::boot_info::MemoryRegions;
use spin::Mutex;

use crate::memory::frame::bit_map_manager::BitMapFrameAllocator;

mod boot_info;
mod bit_map;
pub mod bit_map_manager;

mod frame_init;


pub static FRAME_ALLOCATOR: Mutex<BitMapFrameAllocator> = Mutex::new(BitMapFrameAllocator::uninit());

pub unsafe fn init(memory_regions: &'static MemoryRegions) {
    FRAME_ALLOCATOR.lock().init(memory_regions).expect("Failed Init Allocator");
}