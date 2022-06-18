use crate::test_cap_register;
use crate::usb::xhci::registers::capability::structs::capability_registers_length::CapLength;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;
use crate::utils::test_fn::extract_virtual_mmio_base_addr;


pub trait ICreateCapabilityRegister {
    fn new_capability_length(&self, mmio_base_addr: u64) -> CreateRegisterResult<CapLength>;
}


impl ICreateCapabilityRegister for CreateType {
    fn new_capability_length(&self, mmio_base_addr: u64) -> CreateRegisterResult<CapLength> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(mmio_base_addr) }
        }
    }
}


fn uncheck_transmute(mmio_base: u64) -> CreateRegisterResult<CapLength> {
    let cap_len = transmute_from_u64::<CapLength>(mmio_base);
    Ok(Volatile::Core(RegisterInfo::new(mmio_base, cap_len)))
}

test_cap_register!(should_new_cap_length, uncheck_transmute);
