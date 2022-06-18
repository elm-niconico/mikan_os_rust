use crate::usb::xhci::registers::capability::structs::capability_registers_length::CapLength;
use crate::usb::xhci::registers::capability::structs::db_off::DbOff;
use crate::usb::xhci::registers::capability::structs::hcc_params1::HccParams1;
use crate::usb::xhci::registers::capability::structs::hcc_params2::HccParams2;
use crate::usb::xhci::registers::capability::structs::hci_version::HciVersion;
use crate::usb::xhci::registers::capability::structs::runtime_register_space_offset::RuntimeRegisterSpaceOffset;
use crate::usb::xhci::registers::capability::structs::xhc_parameters1::XhcParameters1;
use crate::usb::xhci::registers::capability::structs::xhc_parameters2::XhcParameters2;
use crate::usb::xhci::registers::capability::structs::xhc_parameters3::XhcParameters3;
use crate::usb::xhci::registers::read_write::volatile::Volatile;

// BAR 1 BAR 0
// Operatational Register addr -> Cap Base + CAP LENGTH
// Runtime Register addr -> CAP BASE + RTS OFF
// Doorbell Register addr -> CAP BASE + DB OFF

#[repr(C)]
#[derive(Debug)]
pub struct CapabilityRegisters {
    pub cap_length: Volatile<CapLength>,
    
    pub hci_version: Volatile<HciVersion>,
    
    pub xhc_params1: Volatile<XhcParameters1>,
    
    pub xhc_params2: Volatile<XhcParameters2>,
    
    pub xhc_params3: Volatile<XhcParameters3>,
    
    pub hcc_params1: Volatile<HccParams1>,
    
    pub db_off: Volatile<DbOff>,
    
    pub rts_off: Volatile<RuntimeRegisterSpaceOffset>,
    
    pub hcc_params2: Volatile<HccParams2>,
}





