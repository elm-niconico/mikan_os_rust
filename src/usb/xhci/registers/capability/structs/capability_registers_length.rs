use bitfield_struct::bitfield;

use crate::impl_debug_from_methods;


#[bitfield(u8)]
pub struct CapLength {
    cap_len: u8,
}


impl_debug_from_methods! {
    CapLength{
        cap_len
    }
}
