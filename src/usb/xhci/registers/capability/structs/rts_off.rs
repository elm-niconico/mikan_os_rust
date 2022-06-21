use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u32)]
pub struct RuntimeRegisterSpaceOffset {
    #[bits(5)]
    _resolve: u8,
    #[bits(27)]
    /** runtime_register_space_offset */
    pub rts_offset: u32,
}
impl_debug_bit_fields! {
    RuntimeRegisterSpaceOffset{
        rts_offset
    }
}
