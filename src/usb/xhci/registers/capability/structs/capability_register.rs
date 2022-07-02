use crate::usb::xhci::registers::capability::structs::capability_registers_length::CapLength;
use crate::usb::xhci::registers::capability::structs::db_off::DoorbellOffsetRegister;
use crate::usb::xhci::registers::capability::structs::hcc_params1::HccParams1;
use crate::usb::xhci::registers::capability::structs::hcc_params2::HccParams2;
use crate::usb::xhci::registers::capability::structs::hci_version::HciVersion;
use crate::usb::xhci::registers::capability::structs::hcs_parameters1::HcsParameters1;
use crate::usb::xhci::registers::capability::structs::hcs_parameters2::HcsParameters2;
use crate::usb::xhci::registers::capability::structs::hcs_parameters3::HcsParameters3;
use crate::usb::xhci::registers::capability::structs::rts_off::RuntimeRegisterSpaceOffset;
use crate::usb::xhci::registers::volatile::Volatile;

// BAR 1 BAR 0
// Operatational Register addr -> Cap Base + CAP LENGTH
// Runtime Register addr -> CAP BASE + RTS OFF
// Doorbell Register addr -> CAP BASE + DB OFF

#[repr(C)]
#[derive(Debug)]
pub struct CapabilityRegisters {
    pub cap_length: Volatile<CapLength>,

    pub hci_version: Volatile<HciVersion>,

    pub hcs_params1: Volatile<HcsParameters1>,

    pub hcs_params2: Volatile<HcsParameters2>,

    pub hcs_params3: Volatile<HcsParameters3>,

    pub hcc_params1: Volatile<HccParams1>,

    pub db_offset: Volatile<DoorbellOffsetRegister>,

    pub rts_offset: Volatile<RuntimeRegisterSpaceOffset>,

    pub hcc_params2: Volatile<HccParams2>,
}