use crate::serial_println;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::operators::structs::device_notification_control::DeviceNotificationControlRegister;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;
use crate::utils::test_fn::extract_operational_base_addr;


pub trait ICreateDeviceNotifyControl {
    fn new_device_notify_control(&self, operational_base_addr: u64) -> CreateRegisterResult<DeviceNotificationControlRegister>;
}

impl ICreateDeviceNotifyControl for CreateType{
    fn new_device_notify_control(&self, operational_base_addr: u64) -> CreateRegisterResult<DeviceNotificationControlRegister> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(operational_base_addr)}
            
            //TODO Notificationのチェック処理
            CreateType::TransmuteWithCheck => {todo!()}
        }
    }
}

fn uncheck_transmute(operational_base_addr: u64) -> CreateRegisterResult<DeviceNotificationControlRegister> {
    let addr = operational_base_addr + 0x14;
    let device_control = transmute_from_u64::<DeviceNotificationControlRegister>(addr);
    
    Ok(RegisterInfo::new(addr, device_control))
}


#[test_case]
pub fn should_new_device_notify_control() {
    let device_notify_control = uncheck_transmute(extract_operational_base_addr());
    
    assert!(device_notify_control.is_ok());
    serial_println!("{:?}", device_notify_control);
}
