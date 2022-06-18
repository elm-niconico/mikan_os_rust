use bitfield_struct::bitfield;
use crate::impl_debug_from_methods;


#[bitfield(u16)]
pub struct VciVersion {
    vci_version: u16,
}

impl_debug_from_methods!{
    VciVersion{
        vci_version
    }
}
