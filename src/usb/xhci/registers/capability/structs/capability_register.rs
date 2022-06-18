use crate::impl_debug_only_fields;
use crate::usb::xhci::registers::capability::structs::capability_registers_length::CapLength;
use crate::usb::xhci::registers::capability::structs::db_off::DbOff;
use crate::usb::xhci::registers::capability::structs::hcc_params2::HccParams2;
use crate::usb::xhci::registers::capability::structs::runtime_register_space_offset::RuntimeRegisterSpaceOffset;
use crate::usb::xhci::registers::capability::structs::vci_version::VciVersion;
use crate::usb::xhci::registers::capability::structs::xhc_parameters1::XhcParameters1;
use crate::usb::xhci::registers::capability::structs::xhc_parameters2::XhcParameters2;
use crate::usb::xhci::registers::capability::structs::xhc_parameters3::XhcParameters3;

// BAR 1 BAR 0
// Operatational Register addr -> Cap Base + CAP LENGTH
// Runtime Register addr -> CAP BASE + RTS OFF
// Doorbell Register addr -> CAP BASE + DB OFF

pub struct CapabilityRegister {
    pub cap_length: CapLength,
    
    _reserve1: bool,
    
    pub hci_version: VciVersion,
    
    pub hcs_params1: XhcParameters1,
    
    pub hcs_params2: XhcParameters2,
    
    pub hcs_params3: XhcParameters3,
    
    pub hcc_params1: XhcParameters1,
    
    pub db_off: DbOff,
    
    pub rts_off: RuntimeRegisterSpaceOffset,
    
    pub hcc_params2: RuntimeRegisterSpaceOffset,
    
    pub vti_os_off: HccParams2,
}

impl_debug_only_fields! {
    CapabilityRegister{
        cap_length,
        hci_version,
        hcs_params1,
        hcs_params2,
        hcs_params3,
        hcc_params1,
        db_off,
        rts_off,
        hcc_params2,
        vti_os_off
    }
}









