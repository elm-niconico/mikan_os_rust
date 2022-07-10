use alloc::sync::Arc;
use core::alloc::Layout;
use core::ptr;
use spin::lock_api::Mutex;

use x86_64::VirtAddr;

use crate::error::KernelResult;
use crate::log;
use crate::memory::heap::HEAP;
use crate::usb::xhci::device::device_context::DeviceContext;
use crate::usb::xhci::device::usb_device::UsbDevice;

#[derive(Debug)]
#[allow(unused)]
pub struct DeviceManager {
    devices: Mutex<*mut UsbDevice>,
    device_contexts: Arc<Mutex<*mut DeviceContext>>,
}

pub trait DeviceContextAddr {
    fn device_context_base_addr(&self) -> VirtAddr;
}


impl DeviceManager {
    #[allow(unused)]
    pub fn try_new(device_max_slots: u8) -> KernelResult<Self> {
        log!("TEST");
        let max_slots = usize::from(device_max_slots) + 1;

        let size = core::mem::size_of::<*mut UsbDevice>();
        let alloc_size = usize::from(size * max_slots);

        log!("Alloc Size {}", alloc_size);
        let l = Layout::from_size_align(alloc_size, 64)?;

        log!("Device Layout {:?}", l);
        let devices = HEAP
            .lock()
            .allocate_first_fit(l)?
            .as_ptr();
        log!("Device PTR{:?}", devices);


        let l = Layout::from_size_align(alloc_size, 64)?;
        log!("Device Context Layout {:?}", l);
        let device_contexts = HEAP
            .lock()
            .allocate_first_fit(l)?.as_ptr();
        log!("Device Context Ptr {:?}", device_contexts);

        Ok(Self {
            devices: Mutex::new(devices as *mut UsbDevice),
            device_contexts: Arc::new(Mutex::new(device_contexts as *mut DeviceContext)),
        })
    }
}

impl Drop for DeviceManager {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(*self.devices.lock());
            ptr::drop_in_place(*self.device_contexts.lock());
        }
    }
}

impl DeviceContextAddr for DeviceManager {
    fn device_context_base_addr(&self) -> VirtAddr {
        VirtAddr::new(self.device_contexts.lock().addr() as u64)
    }
}

#[test_case]
pub fn should_is_ok_when_try_new_device_manager() {
    assert!(DeviceManager::try_new(8).is_ok());
}