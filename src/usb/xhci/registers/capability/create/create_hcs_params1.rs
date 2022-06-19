use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::hcs_parameters1::HcsParameters1;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::utils::raw_ptr::transmute_register;


pub trait ICreateHcsParams1 {
    fn new_hcs_params1(&self, mmio_base_addr: u64) -> CreateRegisterResult<HcsParameters1>;
}


impl ICreateHcsParams1 for CreateType {
    fn new_hcs_params1(&self, mmio_base_addr: u64) -> CreateRegisterResult<HcsParameters1> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(mmio_base_addr) }
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<HcsParameters1> {
    Ok(transmute_register(mmio_base_addr + 0x04))
}


test_cap_register!(should_new_hcs_params1, uncheck_transmute);
