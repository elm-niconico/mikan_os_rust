use crate::serial_println;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::operators::structs::usb_sts::UsbStsRegister;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;
use crate::utils::test_fn::extract_operational_base_addr;


pub trait ICreateUsbStatusRegister {
    fn new_usb_sts(&self, usb_sts_base_addr: u64) -> CreateRegisterResult<UsbStsRegister>;
}


impl ICreateUsbStatusRegister for CreateType {
    fn new_usb_sts(&self, operational_base_addr: u64) -> CreateRegisterResult<UsbStsRegister> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute_usb_sts(operational_base_addr) }
        }
    }
}


fn uncheck_transmute_usb_sts(operational_base: u64) -> CreateRegisterResult<UsbStsRegister> {
    let addr = operational_base + 0x4;
    let usb_sts = transmute_from_u64(addr);
    
    Ok(Volatile::Core(RegisterInfo::new(addr, usb_sts)))
}


fn transmute_with_check_status(operation_base_addr: u64) -> CreateRegisterResult<UsbStsRegister> {
    let addr = operation_base_addr + 0x4;
    let usb_sts = transmute_from_u64::<UsbStsRegister>(addr);
    if usb_sts.host_system_error() && usb_sts.host_controller_error() {
        return Err(());
    }
    
    Ok(Volatile::Core(RegisterInfo::new(addr, usb_sts)))
}


#[test_case]
pub fn should_uncheck_transmute_usb_sts() {
    let usb_sts = uncheck_transmute_usb_sts(extract_operational_base_addr());
    
    assert!(usb_sts.is_ok());
    serial_println!("{:?}", usb_sts);
}

