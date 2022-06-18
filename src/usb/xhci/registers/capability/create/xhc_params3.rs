use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::xhc_parameters3::XhcParameters3;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::utils::raw_ptr::transmute_register;


pub trait ICreateXhcParams3 {
    fn new_xhc_params3(&self, mmio_base_addr: u64) -> CreateRegisterResult<XhcParameters3>;
}


impl ICreateXhcParams3 for CreateType {
    fn new_xhc_params3(&self, mmio_base_addr: u64) -> CreateRegisterResult<XhcParameters3> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(mmio_base_addr) }
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<XhcParameters3> {
    Ok(transmute_register(mmio_base_addr + 0x0C))
}


test_cap_register!(should_new_xhc_params2, uncheck_transmute);
