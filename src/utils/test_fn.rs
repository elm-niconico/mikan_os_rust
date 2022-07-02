use crate::usb::pci::configuration::tmp_find_usb_mouse_base;
use crate::usb::xhci::registers::capability::create::create_all_registers::ICreateAllCapabilityRegisters;
use crate::usb::xhci::registers::capability::structs::capability_register::CapabilityRegisters;
use crate::usb::xhci::registers::capability::structs::capability_registers_length::CapLength;
use crate::usb::xhci::registers::create_type::RegisterCreate;
use crate::usb::xhci::registers::volatile::VolatileRegister;

static OFFSET: u64 = 1649267441664;

#[allow(dead_code)]
pub fn extract_operational_base_addr() -> u64 {
    let mmio = extract_virtual_mmio_base_addr();
    let cap = extract_cap_register(mmio);
    let cap_len: u8 = cap.cap_length.read().into();
    mmio + cap_len as u64
}

#[allow(dead_code)]
pub fn extract_virtual_mmio_base_addr() -> u64 {
    let mmio_base = tmp_find_usb_mouse_base().unwrap();
    mmio_base + OFFSET
}

#[allow(dead_code)]
pub fn extract_runtime_base() -> u64 {
    let mmio = extract_virtual_mmio_base_addr();
    let cap = extract_cap_register(mmio);
    let rts_off = cap.rts_offset.read().rts_offset();
    mmio + rts_off as u64
}

pub fn extract_cap_len(mmio_base: u64) -> CapLength {
    extract_cap_register(mmio_base).cap_length.read()
}

#[allow(dead_code)]
fn extract_cap_register(mmio_base: u64) -> CapabilityRegisters {
    let create = RegisterCreate::UncheckTransmute;
    let capability_registers = create
        .new_capabilities(mmio_base)
        .expect("Failed Mapping to Cap Register");

    capability_registers
}