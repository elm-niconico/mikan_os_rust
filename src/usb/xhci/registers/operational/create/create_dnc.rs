use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb::xhci::registers::operational::structs::device_notification_control::DeviceNotificationControlRegister;
use crate::utils::raw_ptr::transmute_register;


/** Device Notification Control */
pub trait ICreateDnctrl {
    /** New Device Notification Control */
    fn new_dnctrl(&self, operational_base_addr: u64) -> CreateRegisterResult<DeviceNotificationControlRegister>;
}


impl ICreateDnctrl for RegisterCreate {
    fn new_dnctrl(&self, operational_base_addr: u64) -> CreateRegisterResult<DeviceNotificationControlRegister> {
        match self {
            RegisterCreate::UncheckTransmute => { uncheck_transmute(operational_base_addr) }
        }
    }
}


fn uncheck_transmute(operational_base_addr: u64) -> CreateRegisterResult<DeviceNotificationControlRegister> {
    let addr = operational_base_addr + 0x14;
    Ok(transmute_register(addr))
}


#[test_case]
pub fn should_new_device_notify_control() {
    let device_notify_control = uncheck_transmute(crate::utils::test_fn::extract_operational_base_addr());
    
    assert!(device_notify_control.is_ok());
}
