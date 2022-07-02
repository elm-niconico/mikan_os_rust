use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u32)]
pub struct ConfigureRegister {
    pub max_device_slots_enabled: u8,
    pub u3_entry_enable: bool,
    pub configuration_info_enable: bool,
    #[bits(22)]
    _reserve2: u32,
}

impl_debug_bit_fields! {
    ConfigureRegister{
        max_device_slots_enabled,
        u3_entry_enable,
        configuration_info_enable
    }
}
