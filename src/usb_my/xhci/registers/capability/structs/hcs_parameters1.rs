use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u32)]
pub struct HcsParameters1 {
    pub number_of_device_slots: u8,
    
    #[bits(11)]
    pub number_of_interrupts: u16,
    
    #[bits(5)]
    _reserve: u8,
    
    pub number_of_ports: u8,
}
impl_debug_bit_fields! {
    HcsParameters1{
        number_of_device_slots,
        number_of_interrupts,
        number_of_ports
    }
}
