use core::num::NonZeroUsize;

use x86_64::VirtAddr;
use x86_64::structures::paging::{PageSize, PhysFrame, Size4KiB, Translate};

use crate::{FRAME_ALLOCATOR, PAGE_TABLE, serial_println};
use crate::interrupt::identity_mapping;

#[derive(Clone, Debug, Copy)]
pub struct XhcMapper {}

impl XhcMapper {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {}
    }
}

impl xhci::accessor::Mapper for XhcMapper {

    // FIXME Mappingがうまくできていないっぽい
    #[allow(unused)]
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
        let pages = bytes as u64 / Size4KiB::SIZE + 1;
        let base_addr = VirtAddr::new(phys_start as u64).align_down(4096u64).as_u64();
        identity_mapping(&mut *PAGE_TABLE.get_unchecked().lock(), base_addr, pages as usize);

        NonZeroUsize::new(phys_start).unwrap()
    }

    fn unmap(&mut self, virt_start: usize, bytes: usize) {
        serial_println!("Unmap Usb Mouse MMIO {:#?}", virt_start);

        let mut mapper = FRAME_ALLOCATOR.lock();
        let start = PhysFrame::containing_address(unsafe { PAGE_TABLE.get_unchecked().lock().translate_addr(VirtAddr::new(virt_start as u64)) }.unwrap());
        let end = PhysFrame::containing_address(unsafe { PAGE_TABLE.get_unchecked().lock().translate_addr(VirtAddr::new((virt_start + bytes) as u64)) }.unwrap());

        mapper.free(PhysFrame::range(start, end));
    }
}