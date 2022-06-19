use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::hcs_parameters2::HcsParameters2;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::utils::raw_ptr::transmute_register;


pub trait ICreateHcsParams2 {
    fn new_hcs_params2(&self, mmio_base_addr: u64) -> CreateRegisterResult<HcsParameters2>;
}


impl ICreateHcsParams2 for CreateType {
    fn new_hcs_params2(&self, mmio_base_addr: u64) -> CreateRegisterResult<HcsParameters2> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(mmio_base_addr) }
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<HcsParameters2> {
    Ok(transmute_register(mmio_base_addr + 0x08))
}


test_cap_register!(should_new_hcs_params2, uncheck_transmute);
