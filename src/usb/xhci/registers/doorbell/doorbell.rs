use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u32)]
pub struct DoorbellRegister {
    db_target: u8,
    _reserve: u8,
    db_stream_id: u16,
}
impl_debug_bit_fields! {
    DoorbellRegister{
        db_target,
        db_stream_id
    }
}
