use crate::serial_println;
use crate::usb::pci::configuration::tmp_find_usb_mouse_base;
use crate::usb::xhci::registers::capability::structs::capability_register::CapabilityRegister;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::utils::raw_ptr::transmute_from_u64;


pub trait ICapabilityRegisterCreate {
    fn new_capability(&self, mmio_base_address: u64) -> CreateRegisterResult<CapabilityRegister>;
}


impl ICapabilityRegisterCreate for CreateType {
    fn new_capability(&self, mmio_base_address: u64) -> CreateRegisterResult<CapabilityRegister> {
        match self {
            CreateType::UncheckTransmute => { uncheck_transmute(mmio_base_address) }
            
            // TODO Impl With Check
            _ => { todo!() }
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<CapabilityRegister> {
    let capability = transmute_from_u64(mmio_base_addr);
    Ok(Volatile::Core(RegisterInfo::new(mmio_base_addr, capability)))
}


#[test_case]
pub fn should_uncheck_new_cap() {
    let register = uncheck_transmute(tmp_find_usb_mouse_base().unwrap());
    assert!(register.is_ok());
    serial_println!("{:?}", register.unwrap());
}
