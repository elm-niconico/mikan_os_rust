use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::hcs_parameters3::HcsParameters3;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::utils::raw_ptr::transmute_register;

pub trait ICreateHcsParams3 {
    fn new_hcs_params3(&self, mmio_base_addr: u64) -> CreateRegisterResult<HcsParameters3>;
}

impl ICreateHcsParams3 for RegisterCreate {
    fn new_hcs_params3(&self, mmio_base_addr: u64) -> CreateRegisterResult<HcsParameters3> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(mmio_base_addr),
        }
    }
}

fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<HcsParameters3> {
    Ok(transmute_register(mmio_base_addr + 0x0C))
}

test_cap_register!(should_new_hcs_params3, uncheck_transmute);