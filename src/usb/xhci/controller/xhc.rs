use crate::usb::xhci::registers::capability::capability_register::CapabilityRegister;
use crate::usb::xhci::registers::capability::create::register_creator::ICapabilityRegisterCreate;
use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::read_write::volatile::Volatile;


#[allow(dead_code)]
#[derive(Debug)]
pub struct XhcController {
    capability: Volatile<CapabilityRegister>,
}


impl XhcController {
    pub fn new(mmio_base: u64) -> Result<Self, ()> {
        let create = CreateType::UncheckTransmute;
        let capability = create.new_capability(mmio_base)?;
        
        Ok(Self {
            capability
        })
    }
}


