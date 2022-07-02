use core::ops::{Index, IndexMut};
use core::slice;

use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::usb::xhci::registers::runtime::create::create_interrupter_register_set::INTERRUPTER_SET_COUNTS;
use crate::usb::xhci::registers::runtime::structs::interrupter::interrupter_registers::InterrupterRegisterSet;
use crate::usb::xhci::registers::runtime::structs::interrupter::mf_index::MicroFrameIndex;
use crate::usb::xhci::registers::volatile::Volatile;

pub struct InterruptersArray([InterrupterRegisterSet; INTERRUPTER_SET_COUNTS]);

impl InterruptersArray {
    pub fn new(addr: u64) -> Self {
        let ptr = addr as *const InterrupterRegisterSet;

        let interrupters_unknown_size =
            unsafe { slice::from_raw_parts(ptr, INTERRUPTER_SET_COUNTS) };
        let interrupters = pop(interrupters_unknown_size);
        InterruptersArray(*interrupters)
    }
    pub fn primary(&self) -> Volatile<InterrupterRegisterSet> {
        self.get(0)
    }
    pub fn get(&self, index: usize) -> Volatile<InterrupterRegisterSet> {
        let interrupter = &self.0[index];
        let ptr = interrupter as *const InterrupterRegisterSet;

        Volatile::Core(RegisterInfo::<InterrupterRegisterSet>::new(
            ptr.addr() as u64
        ))
    }
}

#[repr(C)]
pub struct RuntimeRegisters {
    /** MicroFrameIndex */
    pub mf_index: Volatile<MicroFrameIndex>,
    pub interrupter_register_set: InterruptersArray,
}

fn pop(barry: &[InterrupterRegisterSet]) -> &[InterrupterRegisterSet; INTERRUPTER_SET_COUNTS] {
    barry.try_into().expect("slice with incorrect length")
}