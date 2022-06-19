use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::runtime::create::create_interrupter_register_set::ICreateInterrupterRegisterSet;
use crate::usb::xhci::registers::runtime::create::create_mf_index::ICreateMfIndex;
use crate::usb::xhci::registers::runtime::structs::runtime_registers::RuntimeRegisters;


pub trait ICreateRuntimeRegisters {
    fn new_runtime_registers(&self, mmio_base: u64, rts_offset: u32) -> Result<RuntimeRegisters, ()>;
}


impl ICreateRuntimeRegisters for CreateType {
    fn new_runtime_registers(&self, mmio_base: u64, rts_offset: u32) -> Result<RuntimeRegisters, ()> {
        let runtime_base = mmio_base + rts_offset as u64;
        let mf_index = self.new_mf_index(runtime_base)?;
        let interrupter_register_set = self.new_interrupter_register_set(runtime_base);
        
        Ok(RuntimeRegisters {
            mf_index,
            interrupter_register_set,
        })
    }
}
