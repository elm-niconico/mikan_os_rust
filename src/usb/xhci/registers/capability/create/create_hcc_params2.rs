use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::hcc_params2::HccParams2;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::utils::raw_ptr::transmute_register;


pub trait ICreateHccParams2 {
    fn new_hcc_params2(&self, mmio_base_addr: u64) -> CreateRegisterResult<HccParams2>;
}


impl ICreateHccParams2 for RegisterCreate {
    fn new_hcc_params2(&self, mmio_base_addr: u64) -> CreateRegisterResult<HccParams2> {
        match self {
            RegisterCreate::UncheckTransmute => { uncheck_transmute(mmio_base_addr) }
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<HccParams2> {
    Ok(transmute_register(mmio_base_addr + 0x1C))
}

test_cap_register!(should_new_unchecked_hcc_params2, uncheck_transmute);

