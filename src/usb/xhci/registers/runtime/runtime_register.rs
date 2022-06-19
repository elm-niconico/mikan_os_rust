use core::fmt::{Debug, Formatter};
use crate::usb::xhci::registers::runtime::micro_frame_index::MicroFrameIndex;


#[repr(packed)]
pub struct RuntimeRegisters {
    pub micro_frame_index: MicroFrameIndex,
    
}
