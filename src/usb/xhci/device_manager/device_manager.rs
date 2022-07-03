use alloc::vec::Vec;
use core::ptr::null_mut;

use crate::usb::xhci::device::device_context::DeviceContext;
use crate::usb::xhci::device::usb_device::UsbDevice;

#[derive(Debug)]
#[allow(unused)]
pub struct DeviceManager {
    devices: Vec<*mut UsbDevice>,
    device_contexts: Vec<*mut DeviceContext>,
}

impl DeviceManager {
    #[allow(unused)]
    pub fn new(device_max_slots: u8) -> Self {
        let max_slots = usize::from(device_max_slots) + 1;
        let mut devices = Vec::<*mut UsbDevice>::with_capacity(max_slots);

        let mut device_contexts = Vec::<*mut DeviceContext>::with_capacity(max_slots);

        unsafe {
            devices.set_len(max_slots);
            device_contexts.set_len(max_slots);
        }

        Self {
            devices,
            device_contexts,
        }
    }

    pub fn get_device_context_arr_raw_ptr(&self) -> u64 {
        self.device_contexts.as_ptr().addr() as u64
    }
}