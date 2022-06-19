use crate::usb::xhci::registers::runtime::structs::interrupter::mf_index::MicroFrameIndex;


#[repr(packed)]
pub struct RuntimeRegisters {
    /** MicroFrameIndex */
    pub mf_index: MicroFrameIndex,
    
}
