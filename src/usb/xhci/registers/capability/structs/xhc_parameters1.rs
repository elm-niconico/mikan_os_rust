use crate::{impl_debug, impl_debug_from_methods};
use bitfield_struct::bitfield;

#[bitfield(u32)]
pub struct XhcParameters1 {
    pub number_of_device_slots: u8,
    
    #[bits(11)]
    pub number_of_interrupts: u16,
    
    #[bits(5)]
    _reserve: u8,
    
    pub number_of_ports: u8,
}
impl_debug_from_methods! {
    XhcParameters1{
        number_of_device_slots,
        number_of_interrupts,
        number_of_ports
    }
}
