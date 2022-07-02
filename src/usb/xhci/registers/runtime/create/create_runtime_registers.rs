use crate::usb::xhci::registers::create_type::RegisterCreate;
use crate::usb::xhci::registers::runtime::create::create_interrupter_register_set::ICreateInterrupterRegisterSet;
use crate::usb::xhci::registers::runtime::create::create_mf_index::ICreateMfIndex;
use crate::usb::xhci::registers::runtime::structs::runtime_registers::RuntimeRegisters;
use crate::utils::error::CommonResult;

pub trait ICreateRuntimeRegisters {
    fn new_runtimes(&self, mmio_base: u64, rts_offset: u32) -> CommonResult<RuntimeRegisters>;
}

impl ICreateRuntimeRegisters for RegisterCreate {
    fn new_runtimes(&self, mmio_base: u64, rts_offset: u32) -> CommonResult<RuntimeRegisters> {
        let runtime_base = mmio_base + rts_offset as u64;
        let mf_index = self.new_mf_index(runtime_base)?;

        let interrupter_register_set = self.new_interrupter_register_set(runtime_base);

        Ok(RuntimeRegisters {
            mf_index,
            interrupter_register_set,
        })
    }
}
// fn tmp(){
//     let runtime_base = mmio_base + rts_offset as u64;
//     let mf_index = self.new_mf_index(runtime_base)?;
//
//     let interrupter_register_set = self.new_interrupter_register_set(runtime_base);
//
//     Ok(RuntimeRegisters {
//         mf_index,
//         interrupter_register_set,
//     })
// }