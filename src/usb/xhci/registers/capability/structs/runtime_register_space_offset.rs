use bitfield_struct::bitfield;
use crate::impl_debug_from_methods;


#[bitfield(u32)]
pub struct RuntimeRegisterSpaceOffset {
    #[bits(5)]
    _resolve: u8,
    #[bits(27)]
    pub runtime_register_space_offset: u32,
}
impl_debug_from_methods! {
    RuntimeRegisterSpaceOffset{
        runtime_register_space_offset
    }
}
