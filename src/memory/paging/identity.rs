use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PhysFrame, Size4KiB, Translate};
use x86_64::structures::paging::mapper::TranslateResult;

use crate::{PAGE_TABLE, serial_println};
use crate::error::kernel_error::KernelResult;

#[allow(unused)]
pub fn virt_to_phys(virt: VirtAddr) -> Option<PhysAddr> {
    serial_println!("Before Virt {:x}", virt.as_u64());
    let tbl = unsafe { PAGE_TABLE.get_unchecked().lock() };
    let a = tbl.translate_addr(virt);
    //let a = Some(PhysAddr::new(virt.as_u64()));
    serial_println!("After Phys {:x}", a.unwrap().as_u64());
    a
}

#[allow(unused)]
pub fn map(rsdp: u64) {
    let mapper = &mut *(unsafe { PAGE_TABLE.get_unchecked() }.lock());
    let frame_allocator = &mut *FRAME_ALLOCATOR.lock();
    let base_addr = VirtAddr::new(rsdp).align_down(4096u64).as_u64();
    make_identity_mapping(mapper, frame_allocator, base_addr, 1).expect("Failed Rsdp Mapping");
}


#[allow(unused)]
/// 恒等変換(Identity Mapping)を行うための機構を提供します。
/// 仮想アドレスと物理アドレスが一致するようにします。
pub fn identity_mapping(
    mapper: &mut x86_64::structures::paging::OffsetPageTable,
    base_addr: u64,
    num_pages: usize,
) -> KernelResult<()> {
    use x86_64::structures::paging::PageTableFlags as Flags;
    let base_page = Page::<Size4KiB>::from_start_address(VirtAddr::new(base_addr))?;
    let base_frame = PhysFrame::<Size4KiB>::from_start_address(PhysAddr::new(base_addr))?;
    let flags = Flags::PRESENT | Flags::WRITABLE;
    for i in 0..num_pages {
        let page = base_page + i as u64;
        let frame = base_frame + i as u64;
        unsafe { mapper.map_to(page, frame, flags, &mut *FRAME_ALLOCATOR.lock()) }?.flush();
    }
    Ok(())
}