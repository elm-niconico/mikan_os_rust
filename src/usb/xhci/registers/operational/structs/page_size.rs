use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u32)]
pub struct PageSizeRegister {
    pub page_size_ro: u16,
    _reserve: u16,
}

impl_debug_bit_fields! {
    PageSizeRegister{
        page_size_ro
    }
}
