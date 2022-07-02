use core::mem;

use bitfield_struct::bitfield;

use crate::usb_my::pci::configuration::{read_conf_reg, Device};

#[bitfield(u32)]
pub struct CapabilityHeader {
    pub cap_id: u8,
    pub next_ptr: u8,
    pub cap: u16,
}

pub struct MsiCapability {
    pub header: MsiCapabilityHeader,
    pub msg_addr: u32,
    pub msg_data: u32,
    pub msg_upper_addr: u32,
    pub mask_bits: u32,
    pub pending_bits: u32,
}

impl MsiCapability {
    pub fn new(dev: &Device, cap_addr: u8) -> Self {
        let header =
            unsafe { mem::transmute::<u32, MsiCapabilityHeader>(read_conf_reg(dev, cap_addr)) };
        Self {
            header,
            msg_addr: 0,
            msg_data: 0,
            msg_upper_addr: 0,
            mask_bits: 0,
            pending_bits: 0,
        }
    }
}

pub union MsiCapabilityHeader {
    pub data: u32,
    pub bits: MsiCapabilityBits,
}

#[bitfield(u32)]
pub struct MsiCapabilityBits {
    pub cap_id: u8,
    pub next_ptr: u8,
    pub msi_enable: bool,
    #[bits(3)]
    pub multi_msg_capable: u8,
    #[bits(3)]
    pub multi_msg_enable: u8,
    pub addr_64_capable: bool,
    pub per_vector_mask_capable: bool,
    #[bits(7)]
    _reserve: u8,
}