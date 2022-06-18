use crate::usb::pci::configuration::tmp_find_usb_mouse_base;
use crate::usb::xhci::registers::capability::structs::vci_version::VciVersion;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::utils::raw_ptr::transmute_register;
use crate::utils::test_fn::extract_virtual_mmio_base_addr;
use crate::{serial_println, test_cap_register};

pub trait ICreateVciVersion {
    fn new_vci_version(&self, mmio_base_address: u64) -> CreateRegisterResult<VciVersion>;
}

impl ICreateVciVersion for CreateType {
    fn new_vci_version(&self, mmio_base_address: u64) -> CreateRegisterResult<VciVersion> {
        match self {
            CreateType::UncheckTransmute => uncheck_transmute(mmio_base_address),
        }
    }
}

fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<VciVersion> {
    Ok(transmute_register::<VciVersion>(mmio_base_addr + 0x02))
}

test_cap_register!(should_new_vci_version, uncheck_transmute);
