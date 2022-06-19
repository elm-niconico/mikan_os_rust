use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u64)]
/** DeviceContextBaseAddressArrayPointerRegister */
pub struct Dcbaap {
    #[bits(6)]
    _reserve: u8,
    
    #[bits(58)]
    /** device_context_base_array_pointer */
    pub dcbaap: u64,
}

impl_debug_bit_fields! {
    Dcbaap{
        dcbaap
    }
}
