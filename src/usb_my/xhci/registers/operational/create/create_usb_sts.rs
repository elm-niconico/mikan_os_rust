use crate::test_op_register;
use crate::usb_my::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb_my::xhci::registers::operational::structs::usb_sts::UsbStsRegister;
use crate::utils::raw_ptr::transmute_register;

pub trait ICreateUsbStatusRegister {
    fn new_usb_sts(&self, usb_sts_base_addr: u64) -> CreateRegisterResult<UsbStsRegister>;
}

impl ICreateUsbStatusRegister for RegisterCreate {
    fn new_usb_sts(&self, operational_base_addr: u64) -> CreateRegisterResult<UsbStsRegister> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(operational_base_addr),
        }
    }
}

fn uncheck_transmute(operational_base: u64) -> CreateRegisterResult<UsbStsRegister> {
    let addr = operational_base + 0x4;
    Ok(transmute_register(addr))
}

test_op_register!(should_new_usb_sts, uncheck_transmute);