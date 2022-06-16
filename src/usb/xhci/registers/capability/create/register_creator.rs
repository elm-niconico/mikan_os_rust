use crate::usb::xhci::registers::capability::capability_register::CapabilityRegister;
use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::utils::raw_ptr::transmute_register;


pub trait ICapabilityRegisterCreate {
    fn new_capability(&self, mmio_base_address: u64) -> Result<Volatile<CapabilityRegister>, ()>;
}


impl ICapabilityRegisterCreate for CreateType {
    fn new_capability(&self, mmio_base_address: u64) -> Result<Volatile<CapabilityRegister>, ()> {
        match self {
            CreateType::UncheckTransmute => { Ok(Volatile::Core(transmute_register::<CapabilityRegister>(mmio_base_address))) }
            
            // TODO Impl With Check
            CreateType::TransmuteWithCheck => { Ok(Volatile::Core(transmute_register::<CapabilityRegister>(mmio_base_address))) }
        }
    }
}


fn transmute_with_check(mmio_base_address: u64) {}
