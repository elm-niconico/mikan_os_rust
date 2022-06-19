use crate::usb::xhci::registers::runtime::structs::micro_frame_index::MicroFrameIndex;


#[repr(packed)]
pub struct RuntimeRegisters {
    /** MicroFrameIndex */
    pub mf_index: MicroFrameIndex,
    
}
