use bitfield_struct::bitfield;


#[allow(dead_code)]
#[bitfield(u32)]
pub struct DeviceNotificationControlRegister {
    pub notification_enable: u16,
    _reserve: u16,
}
