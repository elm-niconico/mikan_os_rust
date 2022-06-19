use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u32)]
// IMAN
pub struct InterrupterManagementRegister {
    // IP
    pub interrupt_pending: bool,
    
    // IE
    pub interrupt_enable: bool,
    
    #[bits(30)]
    _reserve: u32,
}
impl_debug_bit_fields! {
    InterrupterManagementRegister{
        interrupt_pending,
        interrupt_enable
    }
}
