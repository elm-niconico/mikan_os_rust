use bitfield_struct::bitfield;

use crate::impl_debug_bit_filed;


#[bitfield(u64)]
pub struct DeviceContextBaseAddressArrayPointerRegister {
    #[bits(6)]
    _reserve: u8,
    
    #[bits(58)]
    pub device_context_base_array_pointer: u64,
}

impl_debug_bit_filed! {
    DeviceContextBaseAddressArrayPointerRegister{
        device_context_base_array_pointer
    }
}
