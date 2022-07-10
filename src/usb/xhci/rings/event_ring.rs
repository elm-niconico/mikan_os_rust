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


#[repr(align(64))]
struct InterruptArray([EventRingSegmentTableEntry; 1024]);

impl InterruptArray {
    pub fn new() -> Self {
        Self([EventRingSegmentTableEntry::new_zeros(); 1024])
    }
}

#[repr(align(64))]
struct EventDataBuff([TrbBase; 32]);

impl EventDataBuff {
    pub fn new() -> Self {
        Self([TrbBase::new_zeros(); 32])
    }
}

#[allow(unused)]
pub struct EventRing {
    cycle_bit: bool,
    data_buff: *mut [TrbBase],
    erste: *mut [EventRingSegmentTableEntry],
}

impl EventRing {
    pub unsafe fn new() -> Self {
        let data_buff = unsafe { HEAP.alloc_zeroed(Layout::from_size_align(32 * 32, 64).unwrap()) };
        let erste = unsafe { HEAP.alloc_zeroed(Layout::from_size_align_unchecked(128 * 1024, 64)) };
        let mut data_buff = ptr::slice_from_raw_parts_mut(data_buff as *mut TrbBase, 32);
        let mut erste = ptr::slice_from_raw_parts_mut(erste as *mut EventRingSegmentTableEntry, 1024);

        // TODO  Event Ring セグメントサイズの設定


        (*erste)[0].set_ring_segment_base_address((*data_buff).as_ptr().addr() as u64);
        (*erste)[0].set_ring_segment_size(32);

        Self {
            cycle_bit: true,
            data_buff,
            erste,
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
    pub fn has_front(&self, interrupter: &Volatile<InterrupterRegisterSet>) -> bool {
        let trb = self.front_trb(interrupter);
        serial_println!("TRB {:?}", trb);
        trb.cycle_bit() == self.cycle_bit
    }

    #[allow(unused)]
    pub fn front_trb(&self, interrupter: &Volatile<InterrupterRegisterSet>) -> TrbBase {
        let dequeue_ptr = interrupter.get_dequeue_ptr();
        let src = dequeue_ptr;
        unsafe { ptr::read_volatile(src as *const TrbBase) }
    }
}

impl EventRingAddress for EventRing {
    fn dequeue_ptr_addr(&self) -> VirtAddr {
        unsafe { VirtAddr::new((*self.data_buff).as_ptr().addr() as u64) }
    }

    fn segment_tbl_base_addr(&self) -> VirtAddr {
        unsafe { VirtAddr::new((*self.erste).as_ptr().addr() as u64) }
    }
}

impl Drop for EventRing {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(self.data_buff);
            drop_in_place(self.erste);
        }
    }
}