use bitfield_struct::bitfield;

use crate::impl_debug_bit_filed;


#[bitfield(u8)]
pub struct CapLength {
    cap_len: u8,
}


impl_debug_bit_filed! {
    CapLength{
        cap_len
    }
}
