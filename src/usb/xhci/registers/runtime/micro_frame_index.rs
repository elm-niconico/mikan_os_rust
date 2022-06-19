use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u32)]
pub struct MicroFrameIndex {
    #[bits(14)]
    pub micro_frame_index: u16,
    
    #[bits(18)]
    reserve: u32,
}

impl_debug_bit_fields! {
    MicroFrameIndex{
        micro_frame_index
    }
}

