use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::hcc_params1::HccParams1;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::utils::raw_ptr::transmute_register;

pub trait ICreateHccParams1 {
    fn new_hcc_params1(&self, mmio_base_addr: u64) -> CreateRegisterResult<HccParams1>;
}

impl ICreateHccParams1 for RegisterCreate {
    fn new_hcc_params1(&self, mmio_base_addr: u64) -> CreateRegisterResult<HccParams1> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(mmio_base_addr),
        }
    }
}

fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<HccParams1> {
    Ok(transmute_register(mmio_base_addr + 0x10))
}

test_cap_register!(should_new_unchecked_hcc_params1, uncheck_transmute);