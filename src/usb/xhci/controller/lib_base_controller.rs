use core::num::NonZeroUsize;
use core::ops::{Add, AddAssign};
use core::ptr;

use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{Mapper, PageSize, PageTableFlags, PhysFrame, Size4KiB};
use xhci::Registers;

use crate::{FRAME_ALLOCATOR, log, PAGE_TABLE, tmp_find_usb_mouse_base};
use crate::usb::xhci::trb::trb_base::TrbBase;

pub struct LibBaseController {
    registers: Registers<XhcMapper>,
    command_ring: CommandRing,
}


#[repr(align(64))]
struct CommandRing([TrbBase; 32]);


impl LibBaseController {
    pub fn new(phys_offset: VirtAddr) -> Self {
        let mapper = XhcMapper { phys_offset };
        let mut registers = unsafe { Registers::new(tmp_find_usb_mouse_base().unwrap() as usize, mapper) };


        let command_ring = CommandRing([TrbBase::new_zeros(); 32]);
        // registers.operational.crcr.update_volatile(|r| {
        //     let ptr = unsafe { command_ring.0.as_ptr().addr() };
        //     r.set_command_ring_pointer(ptr as u64);
        // });

        Self {
            registers,
            command_ring,
        }
    }


    pub fn process_event(&mut self) {
        let erdp = self.registers.interrupt_register_set.read_volatile_at(0).erdp.event_ring_dequeue_pointer();
        log!("ERDP {}", erdp);
        self.log_usb_sts();
        self.log_usb_cmd();
        let trb_base = unsafe { ptr::read_volatile(erdp as *const TrbBase) };
        log!("TRB BASE {:#?}", trb_base);
    }
    pub fn log_usb_sts(&self) {
        log!("{:#?}", self.registers.operational.usbsts.read_volatile());
    }
    pub fn log_usb_cmd(&self) {
        log!("{:#?}", self.registers.operational.usbcmd.read_volatile());
    }
    pub fn is_run_command_ring(&self) -> bool {
        self.registers.operational.crcr.read_volatile().command_ring_running()
    }
}

#[derive(Clone, Debug)]
struct XhcMapper {
    phys_offset: VirtAddr,
}

impl xhci::accessor::Mapper for XhcMapper {
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
        let mut virt_addr = VirtAddr::new(self.phys_offset.as_u64() + phys_start as u64);

        let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(phys_start as u64));
        let frags = PageTableFlags::WRITABLE | PageTableFlags::PRESENT | PageTableFlags::ACCESSED | PageTableFlags::USER_ACCESSIBLE;

        loop {
            let page = x86_64::structures::paging::Page::<Size4KiB>::containing_address(virt_addr);
            let res = PAGE_TABLE.get().lock().map_to(page, frame, frags, &mut *FRAME_ALLOCATOR.get().lock());
            if let Ok(res) = res {
                res.flush();
                break;
            } else {
                virt_addr.add_assign(Size4KiB::SIZE);
                // page.add_assign(4096);
            }
        }

        NonZeroUsize::new_unchecked(virt_addr.as_u64() as usize)
        //NonZeroUsize::new_unchecked(self.phys_offset.as_u64() as usize + phys_start )
    }

    fn unmap(&mut self, virt_start: usize, bytes: usize) {
        log!("Unmap Usb Mouse MMIO {:#?}", virt_start);
        todo!()
    }
}