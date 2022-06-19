use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::runtime::structs::interrupter::interrupter_registers::InterrupterRegisterSet;
use crate::usb::xhci::registers::runtime::structs::interrupter::mf_index::MicroFrameIndex;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RuntimeRegisters {
    /** MicroFrameIndex */
    pub mf_index: Volatile<MicroFrameIndex>,
    pub interrupter_register_set: [Volatile<InterrupterRegisterSet>; 1024],
}
