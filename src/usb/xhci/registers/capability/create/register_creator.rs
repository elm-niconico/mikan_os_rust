use core::ptr;

use crate::usb::xhci::registers::capability::capability_register::CapabilityRegister;
use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;


pub trait ICapabilityRegisterCreate {
    fn capability_register(&self, mmio_base_address: u64) -> Result<Volatile<CapabilityRegister>, ()>;
}


impl ICapabilityRegisterCreate for CreateType {
    fn capability_register(&self, mmio_base_address: u64) -> Result<Volatile<CapabilityRegister>, ()> {
        match self {
            CreateType::UncheckTransmute => {
                let raw_ptr = mmio_base_address as *const CapabilityRegister;
                let register = unsafe { ptr::read_volatile(raw_ptr) };
                Ok(Volatile::Core(RegisterInfo::new(mmio_base_address, register)))
            }
        }
    }
}
