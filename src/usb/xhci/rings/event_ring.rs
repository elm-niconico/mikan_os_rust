use core::ptr;

use crate::serial_println;
use crate::usb::xhci::registers::runtime::structs::interrupter::interrupter_registers::InterrupterRegisterSet;
use crate::usb::xhci::registers::runtime::structs::runtime_registers::RuntimeRegisters;
use crate::usb::xhci::registers::volatile::{Volatile, VolatileRegister};
use crate::usb::xhci::rings::event_ring_segment_table_entry::EventRingSegmentTableEntry;
use crate::usb::xhci::trb::trb_base::TrbBase;


#[derive(Debug)]
pub struct EventRing {
    cycle_bit: bool,
    physical_offset: u64,
    buff: [u128; 10],
    erste: [EventRingSegmentTableEntry; 1],
}


impl EventRing {
    pub fn new(runtime: &mut RuntimeRegisters, physical_offset: u64) -> Self {
        let buff = [0; 10];
        let mut erste: [EventRingSegmentTableEntry; 1] = [EventRingSegmentTableEntry::new(); 1];
        
        
        let mut interrupter = runtime.interrupter_register_set.primary();
        let buff_addr = buff.as_ptr().addr() as u64;
        
        erste[0].set_ring_segment_size(10);
        erste[0].set_ring_segment_base_address((buff[0] as *const TrbBase).addr() as u64);
        
        interrupter.set_segment_size(1);
        
        interrupter.set_dequeue_ptr(buff_addr);
        
        interrupter.set_segment_tbl_base_addr(erste.as_ptr().addr() as u64);
        
        
        Self {
            cycle_bit: true,
            physical_offset,
            buff,
            erste,
        }
    }
    
    pub fn pop(&mut self, interrupter: &mut Volatile<InterrupterRegisterSet>) {
        let dequeue_ptr: u64 = interrupter.get_dequeue_ptr();
        
        let mut dequeue_ptr = dequeue_ptr + 16;
        let segment_begin = interrupter.read().erstba.erstba();
        let segment_end = segment_begin + interrupter.read().erstsz.erstsz() as u64 + self.physical_offset;
        serial_println!("segment_begin {}", segment_begin);
        serial_println!("segment_end {}", segment_end);
        serial_println!("ptr {}", dequeue_ptr);
        if dequeue_ptr >= segment_end {
            dequeue_ptr = segment_begin;
            
            self.cycle_bit = !self.cycle_bit;
        }
        
        interrupter.set_dequeue_ptr(dequeue_ptr);
    }
    pub fn has_front(&self, interrupter: &Volatile<InterrupterRegisterSet>) -> bool {
        let trb = self.front_trb(interrupter);
        serial_println!("TRB {:?}", trb);
        trb.cycle_bit() == self.cycle_bit
    }
    
    
    pub fn front_trb(&self, interrupter: &Volatile<InterrupterRegisterSet>) -> TrbBase {
        let dequeue_ptr = interrupter.get_dequeue_ptr();
        
        unsafe { ptr::read_volatile(dequeue_ptr as *const TrbBase) }
    }
}
