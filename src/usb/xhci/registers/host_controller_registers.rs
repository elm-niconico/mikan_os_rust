use crate::usb::xhci::registers::capability::create::create_all_registers::ICreateAllCapabilityRegisters;
use crate::usb::xhci::registers::capability::structs::capability_register::CapabilityRegisters;
use crate::usb::xhci::registers::create_type::RegisterCreate;
use crate::usb::xhci::registers::operational::create::create_all_registers::ICreateAllOperationalRegisters;
use crate::usb::xhci::registers::operational::structs::operational_registers::OperationalRegisters;
use crate::usb::xhci::registers::runtime::create::create_runtime_registers::ICreateRuntimeRegisters;
use crate::usb::xhci::registers::runtime::structs::runtime_registers::RuntimeRegisters;
use crate::usb::xhci::registers::volatile::VolatileRegister;


#[derive(Debug)]
pub struct HostControllerRegisters {
    capabilities: CapabilityRegisters,
    operations: OperationalRegisters,
    runtimes: RuntimeRegisters,
}


impl HostControllerRegisters {
    pub fn new(create: RegisterCreate, mmio_base: u64) -> Result<Self, ()> {
        let capabilities = create.new_capabilities(mmio_base)?;
        
        let operations =
            create.new_operations(mmio_base, capabilities.cap_length.read_volatile())?;
        
        let runtimes = create.new_runtimes(mmio_base, capabilities.rts_off.read_volatile().rts_off())?;
        
        Ok(Self {
            capabilities,
            operations,
            runtimes,
        })
    }
    
    
    pub fn capabilities_mut(&mut self) -> &mut CapabilityRegisters {
        &mut self.capabilities
    }
    
    
    pub fn operations_mut(&mut self) -> &mut OperationalRegisters {
        &mut self.operations
    }
    
    
    pub fn runtimes_mut(&mut self) -> &mut RuntimeRegisters {
        &mut self.runtimes
    }
}


