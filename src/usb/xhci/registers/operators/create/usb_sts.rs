use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::operators::structs::usb_sts::UsbStsRegister;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;
use crate::utils::test_fn::extract_operational_base_addr;


pub trait ICreateUsbStatusRegister {
    fn new_usb_sts(&self, usb_sts_base_addr: u64) -> Result<RegisterInfo<UsbStsRegister>, ()>;
}


impl ICreateUsbStatusRegister for CreateType {
    fn new_usb_sts(&self, operational_base_addr: u64) -> Result<RegisterInfo<UsbStsRegister>, ()> {
        match self {
            CreateType::UncheckTransmute => {
                let register_info = RegisterInfo::new(operational_base_addr + 0x04, transmute_usb_sts(operational_base_addr));
                Ok(register_info)
            }
            CreateType::TransmuteWithCheck => { transmute_with_check_status(operational_base_addr) }
        }
    }
}


fn transmute_usb_sts(operational_base: u64) -> UsbStsRegister {
    let addr = operational_base + 0x4;
    transmute_from_u64::<UsbStsRegister>(addr)
}


fn transmute_with_check_status(operation_base_addr: u64) -> Result<RegisterInfo<UsbStsRegister>, ()> {
    let addr = operation_base_addr + 0x4;
    let usb_sts = transmute_from_u64::<UsbStsRegister>(addr);
    if usb_sts.host_system_error() && usb_sts.host_controller_error() {
        return Err(());
    }
    
    Ok(RegisterInfo::new(addr, usb_sts))
}


#[test_case]
pub fn should_transmute_usb_sts() {
    let usb_sts = transmute_with_check_status(extract_operational_base_addr());
    
    
    assert!(usb_sts.is_ok());
}

