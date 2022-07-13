use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use core::ptr::drop_in_place;

use x86_64::VirtAddr;

use crate::memory::heap::HEAP;
use crate::serial_println;
use crate::usb::xhci::registers::runtime::structs::interrupter::interrupter_registers::InterrupterRegisterSet;
use crate::usb::xhci::registers::volatile::Volatile;
use crate::usb::xhci::rings::event_ring_segment_table_entry::EventRingSegmentTableEntry;
use crate::usb::xhci::trb::trb_base::TrbBase;

pub trait EventRingAddress {
    fn dequeue_ptr_addr(&self) -> VirtAddr;
    fn segment_tbl_base_addr(&self) -> VirtAddr;
}


#[allow(unused)]
pub struct EventRing {
    cycle_bit: bool,
    event_ring_segment: *mut [u128],
    event_ring_segment_tbl: *mut [EventRingSegmentTableEntry],
}

impl EventRing {
    pub unsafe fn new() -> Self {
        let event_ring_segment_tbl = unsafe { HEAP.alloc_zeroed(Layout::from_size_align_unchecked(128, 64)) };
        let event_ring_segment = unsafe { HEAP.alloc_zeroed(Layout::from_size_align(128 * 32, 64).unwrap()) };

        let event_ring_segment_tbl = ptr::slice_from_raw_parts_mut(event_ring_segment_tbl as *mut EventRingSegmentTableEntry, 1);
        let event_ring_segment = ptr::slice_from_raw_parts_mut(event_ring_segment as *mut u128, 32);

        (*event_ring_segment_tbl)[0].set_ring_segment_base_address((*event_ring_segment).as_ptr().addr() as u64);
        (*event_ring_segment_tbl)[0].set_ring_segment_size(32);

        Self {
            cycle_bit: true,
            event_ring_segment,
            event_ring_segment_tbl,
        }
    }


    #[allow(unused)]
    pub fn pop(&mut self, interrupter: &mut Volatile<InterrupterRegisterSet>) {
        // let dequeue_ptr: u64 = interrupter.get_dequeue_ptr();
        //
        // let mut dequeue_ptr = dequeue_ptr + 16;
        // let segment_begin = interrupter.read().erstba.erstba();
        // let segment_end =
        //     segment_begin + interrupter.read().erstsz.erstsz() as u64 + self.physical_offset;
        // // serial_println!("segment_begin {}", segment_begin);
        // // serial_println!("segment_end {}", segment_end);
        // // serial_println!("ptr {}", dequeue_ptr);
        // if dequeue_ptr >= segment_end {
        //     dequeue_ptr = segment_begin;
        //
        //     self.cycle_bit = !self.cycle_bit;
        // }
        //
        // interrupter.set_dequeue_ptr(dequeue_ptr);
    }
    #[allow(unused)]
    pub fn has_front(&self, interrupter: xhci::registers::InterruptRegisterSet, offset: u64) -> bool {
        let trb = self.front_trb(interrupter, offset, 0);
        serial_println!("trb {:?}",trb);
        if trb.trb_type() != 0 {}


        trb.cycle_bit() == self.cycle_bit
    }

    #[allow(unused)]
    pub fn front_trb(&self, interrupter: xhci::registers::InterruptRegisterSet, offset: u64, o: isize) -> TrbBase {
        let dequeue_ptr = interrupter.erdp.event_ring_dequeue_pointer() + offset;

        unsafe { ptr::read_volatile(self.event_ring_segment as *mut TrbBase) }
    }
}

impl EventRingAddress for EventRing {
    fn dequeue_ptr_addr(&self) -> VirtAddr {
        unsafe {
            VirtAddr::new((*self.event_ring_segment).as_ptr().addr() as u64)
        }
    }

    fn segment_tbl_base_addr(&self) -> VirtAddr {
        unsafe { VirtAddr::new((*self.event_ring_segment_tbl).as_ptr().addr() as u64) }
    }
}

impl Drop for EventRing {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(self.event_ring_segment);
            drop_in_place(self.event_ring_segment_tbl);
        }
    }
}