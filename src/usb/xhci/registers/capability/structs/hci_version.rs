use bitfield_struct::bitfield;

use crate::impl_debug_bit_filed;


#[bitfield(u16)]
pub struct HciVersion {
    hci_version: u16,
}

impl_debug_bit_filed! {
    HciVersion{
        hci_version
    }
}
