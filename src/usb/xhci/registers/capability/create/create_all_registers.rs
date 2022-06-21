use crate::usb::xhci::registers::capability::create::create_cap_length::ICreateCapLength;
use crate::usb::xhci::registers::capability::create::create_db_off::ICreateDbOff;
use crate::usb::xhci::registers::capability::create::create_hcc_params1::ICreateHccParams1;
use crate::usb::xhci::registers::capability::create::create_hcc_params2::ICreateHccParams2;
use crate::usb::xhci::registers::capability::create::create_hci_version::ICreateVciVersion;
use crate::usb::xhci::registers::capability::create::create_hcs_params1::ICreateHcsParams1;
use crate::usb::xhci::registers::capability::create::create_hcs_params2::ICreateHcsParams2;
use crate::usb::xhci::registers::capability::create::create_hcs_params3::ICreateHcsParams3;
use crate::usb::xhci::registers::capability::create::create_rts_off::ICreateRtsOff;
use crate::usb::xhci::registers::capability::structs::capability_register::CapabilityRegisters;
use crate::usb::xhci::registers::create_type::RegisterCreate;
use crate::utils::error::CommonResult;


pub trait ICreateAllCapabilityRegisters {
    fn new_capabilities(&self, mmio_base_address: u64) -> CommonResult<CapabilityRegisters>;
}


impl ICreateAllCapabilityRegisters for RegisterCreate {
    fn new_capabilities(&self, mmio_base_address: u64) -> CommonResult<CapabilityRegisters> {
        let cap_length = self.new_capability_length(mmio_base_address)?;
        let hci_version = self.new_hci_version(mmio_base_address)?;
        let hcs_params1 = self.new_hcs_params1(mmio_base_address)?;
        let hcs_params2 = self.new_hcs_params2(mmio_base_address)?;
        let hcs_params3 = self.new_hcs_params3(mmio_base_address)?;
        let hcc_params1 = self.new_hcc_params1(mmio_base_address)?;
        let db_off = self.new_db_off(mmio_base_address)?;
        let rts_off = self.new_rts_off(mmio_base_address)?;
        let hcc_params2 = self.new_hcc_params2(mmio_base_address)?;
        Ok(CapabilityRegisters {
            cap_length,
            hci_version,
            hcs_params1,
            hcs_params2,
            hcs_params3,
            hcc_params1,
            db_offset: db_off,
            rts_offset: rts_off,
            hcc_params2,
        })
    }
}


#[test_case]
pub fn should_uncheck_new_cap() {
    let cap_registers = RegisterCreate::UncheckTransmute
        .new_capabilities(crate::utils::test_fn::extract_virtual_mmio_base_addr());
    assert!(cap_registers.is_ok());
}
