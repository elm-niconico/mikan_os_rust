use core::alloc::Layout;
use core::ptr;

use x86_64::VirtAddr;

use crate::error::KernelResult;
use crate::memory::heap::HEAP;
use crate::usb::xhci::device::device_context::DeviceContext;
use crate::usb::xhci::device::usb_device::UsbDevice;

#[derive(Debug)]
#[allow(unused)]
pub struct DeviceManager {
    devices: *mut *mut UsbDevice,
    device_contexts: *mut *mut DeviceContext,
}

pub trait DeviceContextAddr {
    fn device_context_base_addr(&self) -> VirtAddr;
}


impl DeviceManager {
    #[allow(unused)]
    pub fn try_new(device_max_slots: u8) -> KernelResult<Self> {
        let max_slots = usize::from(device_max_slots) + 1;

        let size = core::mem::size_of::<*mut UsbDevice>();
        let alloc_size = usize::from(16 * max_slots);


        let l = Layout::from_size_align(alloc_size, 64)?;


        let devices = HEAP
            .lock()
            .allocate_first_fit(l)?
            .as_ptr();


        let l = Layout::from_size_align(alloc_size, 64)?;

        let device_contexts = HEAP
            .lock()
            .allocate_first_fit(l)?.as_ptr();

        Ok(Self {
            devices: devices as *mut *mut UsbDevice,
            device_contexts: device_contexts as *mut *mut DeviceContext,
        })
    }

    pub fn set(&mut self, deivceContext: *mut DeviceContext) {
        unsafe { self.device_contexts.write_volatile(deivceContext) }
    }
}

impl Drop for DeviceManager {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.devices);
            ptr::drop_in_place(self.device_contexts);
        }
    }
}

impl DeviceContextAddr for DeviceManager {
    fn device_context_base_addr(&self) -> VirtAddr {
        VirtAddr::new(self.device_contexts.addr() as u64)
    }
}

#[test_case]
pub fn should_is_ok_when_try_new_device_manager() {
    assert!(DeviceManager::try_new(8).is_ok());
}