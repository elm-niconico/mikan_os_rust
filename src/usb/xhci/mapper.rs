use core::num::NonZeroUsize;

use crate::{map, serial_println};

#[derive(Clone, Debug)]
pub struct XhcMapper {}

impl XhcMapper {
    pub fn new() -> Self {
        Self {}
    }
}

impl xhci::accessor::Mapper for XhcMapper {
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
        // use x86_64::structures::paging::Mapper as Mapping;
        //serial_println!("Start Mapping {}", phys_start);
        map(phys_start as u64);


        //  return NonZeroUsize::new(phys_start + self.phys_offset as usize).unwrap();
        // let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new((phys_start) as u64 + self.phys_offset));
        //
        // serial_println!("Frame ");
        // let frags = PageTableFlags::WRITABLE | PageTableFlags::PRESENT;
        // let allocator = &mut *FRAME_ALLOCATOR.get().lock();
        //
        //
        // let mut offset_table = PAGE_TABLE.get().lock();
        // if let Ok(unmap) = offset_table.unmap(Page::<Size4KiB>::containing_address(VirtAddr::new(phys_start as u64))) {
        //     unmap.1.flush();
        // }
        //
        // serial_println!("Mapping");
        // offset_table.identity_map(frame, frags, allocator).expect("Failed Identity Mapping").flush();
        // //
        // serial_println!("Non Zeros");

        NonZeroUsize::new(phys_start).unwrap()
    }

    fn unmap(&mut self, virt_start: usize, bytes: usize) {
        serial_println!("Unmap Usb Mouse MMIO {:#?}", virt_start);
        // TODO Un Map
    }
}