use bitfield_struct::bitfield;
use crate::impl_debug_bit_fields;


#[bitfield(u64)]
pub struct EventRingSegmentTableBaseAddressRegister{
    #[bits(6)]
    _reserve: u8,
    
    #[bits(58)]
    /** EventRingSegmentTableBaseAddress */
    pub erstba: u64
}

impl_debug_bit_fields!{
    EventRingSegmentTableBaseAddressRegister{
        erstba
    }
}
