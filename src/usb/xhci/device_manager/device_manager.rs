use alloc::vec::Vec;

use crate::usb::xhci::device::device_context::DeviceContext;
use crate::usb::xhci::device::usb_device::UsbDevice;


#[derive(Debug)]
pub struct DeviceManager {
    devices: Vec<UsbDevice>,
    device_contexts: Vec<DeviceContext>,
}


impl DeviceManager {
    pub fn new(device_max_slots: u8) -> Self {
        let max_slots = usize::from(device_max_slots);
        let devices = Vec::<UsbDevice>::with_capacity(max_slots + 1);
        let device_contexts = Vec::<DeviceContext>::with_capacity(max_slots + 1);
        Self {
            devices,
            device_contexts,
        }
    }
    
    pub fn get_device_context_arr_raw_ptr(&self) -> u64 {
        self.device_contexts.as_ptr().addr() as u64
    }
}
