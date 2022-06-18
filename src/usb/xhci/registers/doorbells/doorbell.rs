use bitfield_struct::bitfield;

use crate::impl_debug_from_methods;


#[bitfield(u32)]
pub struct DoorbellRegister {
    db_target: u8,
    _reserve: u8,
    db_stream_id: u16,
}
impl_debug_from_methods! {
    DoorbellRegister{
        db_target,
        db_stream_id
    }
}