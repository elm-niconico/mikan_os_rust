use crate::test_cap_register;
use crate::usb_my::xhci::registers::capability::structs::capability_registers_length::CapLength;
use crate::usb_my::xhci::registers::create_type::{CreateRegisterResult, RegisterCreate};
use crate::usb_my::xhci::registers::register_info::RegisterInfo;
use crate::usb_my::xhci::registers::volatile::Volatile;

pub trait ICreateCapLength {
    fn new_capability_length(&self, mmio_base_addr: u64) -> CreateRegisterResult<CapLength>;
}

impl ICreateCapLength for RegisterCreate {
    fn new_capability_length(&self, mmio_base_addr: u64) -> CreateRegisterResult<CapLength> {
        match self {
            RegisterCreate::UncheckTransmute => uncheck_transmute(mmio_base_addr),
        }
    }
}

fn uncheck_transmute(mmio_base: u64) -> CreateRegisterResult<CapLength> {
    Ok(Volatile::Core(RegisterInfo::new(mmio_base)))
}

test_cap_register!(should_new_cap_length, uncheck_transmute);