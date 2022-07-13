use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PhysFrame, Size4KiB, Translate};
use x86_64::structures::paging::mapper::TranslateResult;

use crate::{PAGE_TABLE, serial_println};
use crate::error::KernelResult;

#[allow(unused)]
pub fn virt_to_phys(virt: VirtAddr) -> Option<PhysAddr> {
    serial_println!("Before Virt {:x}", virt.as_u64());
    let tbl = unsafe { PAGE_TABLE.get_unchecked().lock() };
    let a = tbl.translate_addr(virt);
    //let a = Some(PhysAddr::new(virt.as_u64()));
    serial_println!("After Phys {:x}", a.unwrap().as_u64());
    a
}

// #[allow(unused)]
// pub fn phys_to_virt(phys: PhysAddr) -> VirtAddr{
//     let tbl = unsafe { PAGE_TABLE.get_unchecked().lock() };
//     tbl.
// }
/// 恒等変換(Identity Mapping)を行うための機構を提供します。
/// 仮想アドレスと物理アドレスが一致するようにします。
#[allow(unused)]
pub fn make_identity_mapping(
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

        if let TranslateResult::Mapped { .. } = mapper.translate(page.start_address()) {
            mapper.unmap(page).unwrap().1.flush();
        };

        unsafe { mapper.map_to(page, frame, flags, &mut *allocator) }?.flush();
    }

    Ok(())
}