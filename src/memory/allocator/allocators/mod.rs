use linked_list_allocator::LockedHeap;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::page::PageRange;
use x86_64::VirtAddr;

#[global_allocator]
pub static HEAP: LockedHeap = LockedHeap::empty();

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 10024; // 100 KiB

pub(crate) fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let heap_page_range = extract_heap_page_range();
    for heap_page in heap_page_range {
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        unsafe {
            mapper
                .map_to(heap_page, frame, flags, frame_allocator)?
                .flush();
        };
    }

    unsafe {
        HEAP.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

fn extract_heap_page_range() -> PageRange {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_end = VirtAddr::new((HEAP_START + HEAP_SIZE) as u64);
    Page::range(
        Page::containing_address(heap_start),
        Page::containing_address(heap_end),
    )
}