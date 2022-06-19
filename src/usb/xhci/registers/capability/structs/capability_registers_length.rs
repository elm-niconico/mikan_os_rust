use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u8)]
pub struct CapLength {
    cap_len: u8,
}


impl_debug_bit_fields! {
    CapLength{
        cap_len
    }
}
