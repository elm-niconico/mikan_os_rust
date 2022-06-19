use bitfield_struct::bitfield;


#[bitfield(u32)]
/** IMOD */
pub struct InterrupterModerationRegister {
    /** interrupt_moderation_interval  */
    pub imodi: u16,
    
    /** interrupt_moderation_counter */
    pub imodc: u16,
}
