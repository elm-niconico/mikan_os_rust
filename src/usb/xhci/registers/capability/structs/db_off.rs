use crate::impl_debug_bit_fields;
use bitfield_struct::bitfield;

#[bitfield(u32)]
// DoorbellRegisterのオフセットを示す
pub struct DoorbellOffsetRegister {
    #[bits(2)]
    _resolved: u8,
    
    #[bits(30)]
    // DoorbellRegisterのアドレス = CAP_BASE + doorbell_array_offset
    pub doorbell_array_offset: u32,
}
impl_debug_bit_fields! {
    DoorbellOffsetRegister{
        doorbell_array_offset
    }
}
