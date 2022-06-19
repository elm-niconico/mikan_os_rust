use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::capability_registers_length::CapLength;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb::xhci::registers::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;


pub trait ICreateCapLength {
    fn new_capability_length(&self, mmio_base_addr: u64) -> CreateRegisterResult<CapLength>;
}


impl ICreateCapLength for RegisterCreate {
    fn new_capability_length(&self, mmio_base_addr: u64) -> CreateRegisterResult<CapLength> {
        match self {
            RegisterCreate::UncheckTransmute => { uncheck_transmute(mmio_base_addr) }
        }
    }
}


fn uncheck_transmute(mmio_base: u64) -> CreateRegisterResult<CapLength> {
    let cap_len = transmute_from_u64::<CapLength>(mmio_base);
    Ok(Volatile::Core(RegisterInfo::new(mmio_base, cap_len)))
}

test_cap_register!(should_new_cap_length, uncheck_transmute);
