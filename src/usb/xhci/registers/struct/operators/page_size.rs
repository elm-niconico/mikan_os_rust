use crate::impl_debug_from_methods;
use bitfield_struct::bitfield;

#[allow(dead_code)]
#[bitfield(u32)]
pub struct PageSizeRegister {
    pub page_size_ro: u16,
    _reserve: u16,
}

impl_debug_from_methods! {
    PageSizeRegister{
        page_size_ro
    }
}
