use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PhysFrame, Size4KiB};

use crate::error::KernelResult;

/// 恒等変換(Identity Mapping)を行うための機構を提供します。
/// 仮想アドレスと物理アドレスが一致するようにします。
pub(crate) fn make_identity_mapping(
    mapper: &mut OffsetPageTable,
    allocator: &mut impl FrameAllocator<Size4KiB>,
    base_addr: u64,
    num_pages: usize,
) -> KernelResult<()> {
    use x86_64::structures::paging::PageTableFlags as Flags;
    let base_page = Page::<Size4KiB>::from_start_address(VirtAddr::new(base_addr))?;
    let base_frame = PhysFrame::from_start_address(PhysAddr::new(base_addr))?;
    let flags = Flags::PRESENT | Flags::WRITABLE;
    for i in 0..num_pages {
        let page = base_page + i as u64;
        let frame = base_frame + i as u64;
        unsafe { mapper.map_to(page, frame, flags, &mut *allocator) }?.flush();
    }

    Ok(())
}