use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u16)]
pub struct HciVersion {
    hci_version: u16,
}

impl_debug_bit_fields! {
    HciVersion{
        hci_version
    }
}
