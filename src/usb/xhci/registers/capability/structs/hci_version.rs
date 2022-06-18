use bitfield_struct::bitfield;

use crate::impl_debug_from_methods;


#[bitfield(u16)]
pub struct HciVersion {
    hci_version: u16,
}

impl_debug_from_methods! {
    HciVersion{
        hci_version
    }
}
