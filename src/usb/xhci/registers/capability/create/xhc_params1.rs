use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::xhc_parameters1::XhcParameters1;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::utils::raw_ptr::transmute_register;


pub trait ICreateXhcParams1 {
    fn new_xhc_params1(&self, mmio_base_addr: u64) -> CreateRegisterResult<XhcParameters1>;
}


impl ICreateXhcParams1 for CreateType {
    fn new_xhc_params1(&self, mmio_base_addr: u64) -> CreateRegisterResult<XhcParameters1> {
        match self {
            CreateType::UncheckTransmute => {uncheck_transmute(mmio_base_addr)}
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<XhcParameters1> {
    Ok(transmute_register(mmio_base_addr + 0x04))
}


test_cap_register!(should_new_xhc_params1, uncheck_transmute);
