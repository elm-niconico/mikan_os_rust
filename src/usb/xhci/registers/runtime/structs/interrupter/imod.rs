use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u32)]
/** IMOD */
pub struct InterrupterModerationRegister {
    /** interrupt_moderation_interval  */
    pub imodi: u16,
    
    /** interrupt_moderation_counter */
    pub imodc: u16,
}
impl_debug_bit_fields! {
    InterrupterModerationRegister{
        imodi,
        imodc
    }
}
