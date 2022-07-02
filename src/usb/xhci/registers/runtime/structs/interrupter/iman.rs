use bitfield_struct::bitfield;
use crate::impl_debug_bit_fields;


#[bitfield(u32)]
/** IMAN */
pub struct InterrupterManagementRegister {
    /** interrupt_pending */
    pub ip: bool,
    
    /** interrupt_enable */
    pub ie: bool,
    
    #[bits(30)]
    _reserve: u32,
}
impl_debug_bit_fields! {
    InterrupterManagementRegister{
        ip,
        ie
    }
}
