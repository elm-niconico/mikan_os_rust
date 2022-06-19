use bitfield_struct::bitfield;

use crate::impl_debug_from_methods;


#[bitfield(u32)]
pub struct DeviceNotificationControlRegister {
    pub notification_enable: u16,
    _reserve: u16,
}


impl_debug_from_methods! {
    DeviceNotificationControlRegister{
        notification_enable
    }
}
