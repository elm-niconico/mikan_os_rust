use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u32)]
pub struct DeviceNotificationControlRegister {
    pub notification_enable: u16,
    _reserve: u16,
}


impl_debug_bit_fields! {
    DeviceNotificationControlRegister{
        notification_enable
    }
}
