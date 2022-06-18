use crate::{serial_println, test_cap_register};
use crate::usb::xhci::registers::capability::structs::hci_version::HciVersion;
use crate::usb::xhci::registers::create_type::{CreateRegisterResult, CreateType};
use crate::utils::raw_ptr::transmute_register;
use crate::utils::test_fn::extract_virtual_mmio_base_addr;


pub trait ICreateVciVersion {
    fn new_hci_version(&self, mmio_base_address: u64) -> CreateRegisterResult<HciVersion>;
}


impl ICreateVciVersion for CreateType {
    fn new_hci_version(&self, mmio_base_address: u64) -> CreateRegisterResult<HciVersion> {
        match self {
            CreateType::UncheckTransmute => uncheck_transmute(mmio_base_address),
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64) -> CreateRegisterResult<HciVersion> {
    Ok(transmute_register::<HciVersion>(mmio_base_addr + 0x02))
}

test_cap_register!(should_new_hci_version, uncheck_transmute);
