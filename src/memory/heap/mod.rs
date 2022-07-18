use linked_list_allocator::LockedHeap;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::page::PageRange;
use x86_64::VirtAddr;

mod dummy;

#[global_allocator]
pub static HEAP: LockedHeap = LockedHeap::empty();