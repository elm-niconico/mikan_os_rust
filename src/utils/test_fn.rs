use crate::usb::pci::configuration::tmp_find_usb_mouse_base;
use crate::usb::xhci::registers::capability::capability_register::CapabilityRegister;
use crate::usb::xhci::registers::capability::create::register_creator::ICapabilityRegisterCreate;
use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::read_write::volatile::IVolatile;


static OFFSET: u64 = 1649267441664;


#[allow(dead_code)]
pub fn extract_operational_base_addr() -> u64 {
    let mmio = extract_virtual_mmio_base_addr();
    let cap = extract_cap_register(mmio);
    let cap_len: u8 = cap.cap_length.into();
    mmio + cap_len as u64
}


#[allow(dead_code)]
fn extract_virtual_mmio_base_addr() -> u64 {
    let mmio_base = tmp_find_usb_mouse_base().unwrap();
    mmio_base + OFFSET
}


#[allow(dead_code)]
fn extract_cap_register(mmio_base: u64) -> CapabilityRegister {
    let create = CreateType::UncheckTransmute;
    let volatile = create
        .new_capability(mmio_base)
        .expect("Failed Mapping to Cap Register");
    
    volatile.read_volatile()
}
