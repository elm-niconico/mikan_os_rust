use alloc::vec;
use alloc::vec::Vec;
use crate::usb::pci::configuration::Device;
use crate::usb::xhci::device::usb_device::UsbDevice;


pub struct DeviceManager {
    devices: Vec<UsbDevice>,
}


impl DeviceManager {
    pub fn new(device_max_slots: u8) -> Self {
        let max_slots = usize::from(device_max_slots);
        let devices = Vec::<UsbDevice>::with_capacity(max_slots);
        Self {
            devices
        }
    }
}
