use crate::test_op_register;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb::xhci::registers::operational::structs::usb_sts::UsbStsRegister;
use crate::usb::xhci::registers::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;


pub trait ICreateUsbStatusRegister {
    fn new_usb_sts(&self, usb_sts_base_addr: u64) -> CreateRegisterResult<UsbStsRegister>;
}


impl ICreateUsbStatusRegister for RegisterCreate {
    fn new_usb_sts(&self, operational_base_addr: u64) -> CreateRegisterResult<UsbStsRegister> {
        match self {
            RegisterCreate::UncheckTransmute => { uncheck_transmute(operational_base_addr) }
        }
    }
}


fn uncheck_transmute(operational_base: u64) -> CreateRegisterResult<UsbStsRegister> {
    let addr = operational_base + 0x4;
    let usb_sts = transmute_from_u64(addr);
    
    Ok(Volatile::Core(RegisterInfo::new(addr, usb_sts)))
}

test_op_register!(should_new_usb_sts, uncheck_transmute);
