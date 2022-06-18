use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::xhc_parameters1::XhcParameters1;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::utils::raw_ptr::transmute_register;


pub trait ICreateXhcParams3 {
    fn new_xhc_params3(&self, mmio_base_addr: u64) -> CreateRegisterResult<XhcParameters1>;
}


impl ICreateXhcParams3 for CreateType {
    fn new_xhc_params3(&self, mmio_base_addr: u64) -> CreateRegisterResult<XhcParameters1> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(mmio_base_addr) }
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<XhcParameters1> {
    Ok(transmute_register(mmio_base_addr + 0x0C))
}


test_cap_register!(should_new_xhc_params2, uncheck_transmute);
