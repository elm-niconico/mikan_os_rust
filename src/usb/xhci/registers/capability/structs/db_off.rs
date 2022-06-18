use crate::impl_debug_from_methods;
use bitfield_struct::bitfield;

#[bitfield(u32)]
pub struct DbOff {
    #[bits(2)]
    _resolved: u8,
    
    #[bits(30)]
    // DoorbellRegisterのアドレス = CAP_BASE + doorbel_array_offset
    pub doorbell_array_offset: u32,
}
impl_debug_from_methods! {
    DbOff{
        doorbell_array_offset
    }
}
