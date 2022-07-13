use core::alloc::Layout;

use x86_64::PhysAddr;

use crate::error::{DeviceContextErrInfo, KernelError, KernelResult};
use crate::memory::heap::HEAP;
use crate::usb::xhci::controller::initialize::XhcInitializer;
use crate::usb::xhci::controller::lib_base_controller::LibBaseController;
use crate::usb::xhci::device::device_context::DeviceContext;

impl XhcInitializer for LibBaseController {
    fn wait_xhc_halted(&mut self) -> KernelResult<()> {
        self.registers.operational.usbcmd.update_volatile(|r| {
            r.clear_interrupter_enable();
            r.clear_host_system_error_enable();
            r.clear_enable_wrap_event();
        });

        if !self.registers.operational.usbsts.read_volatile().hc_halted() {
            self.registers.operational.usbcmd.update_volatile(|r| {
                r.clear_run_stop();
            });
        }

        while !self.registers.operational.usbsts.read_volatile().hc_halted() {}

        if self.registers.operational.usbcmd.read_volatile().run_stop() {
            Err(KernelError::FailedHcHalted)
        } else {
            Ok(())
        }
    }

    fn reset_controller(&mut self) -> KernelResult<()> {
        self.registers.operational.usbcmd.update_volatile(|r| {
            r.set_host_controller_reset();
        });

        while self.registers.operational.usbcmd.read_volatile().host_controller_reset() {}

        while self.registers.operational.usbsts.read_volatile().controller_not_ready() {}

        if self.registers.operational.usbcmd.read_volatile().run_stop() {
            Err(KernelError::FailedHcReset)
        } else {
            Ok(())
        }
    }

    fn set_max_slots(&mut self, device_max_slots: u8) -> KernelResult<()> {
        let limit_slots = self.registers.capability.hcsparams1.read_volatile().number_of_device_slots();
        if limit_slots < device_max_slots {
            return Err(KernelError::OverFlowDeviceMaxSlots);
        }

        self.registers.operational.config.update_volatile(|r| {
            r.set_max_device_slots_enabled(device_max_slots);
        });

        Ok(())
    }

    fn set_scratchpad_buff(&mut self) {
        let max_scratch = self.registers.capability.hcsparams2.read_volatile().max_scratchpad_buffers();
        let buff = unsafe { HEAP.lock().allocate_first_fit(Layout::from_size_align_unchecked((4096 * max_scratch) as usize, 4096)) }
            .expect("Failed Alloc Scratch")
            .as_ptr();
        self.device_manager.set(buff as *mut DeviceContext)
    }

    fn set_device_context_base_addr(&mut self, device_context_addr: PhysAddr) -> KernelResult<()> {
        self.registers.operational.dcbaap.update_volatile(|r| {
            r.set(device_context_addr.as_u64());
        });

        let expect_set_dcbaddr = device_context_addr.as_u64() & !0b1111u64;
        let actual = self.registers.operational.dcbaap.read_volatile().get();
        if actual == expect_set_dcbaddr {
            Ok(())
        } else {
            Err(KernelError::FailedSetDeviceContextBase(DeviceContextErrInfo {
                expect: expect_set_dcbaddr,
                actual,
            }))
        }
    }

    fn register_command_ring(&mut self, command_ring_addr: PhysAddr) -> KernelResult<()> {
        self.registers.operational.crcr.update_volatile(|r| {
            r.set_command_ring_pointer(command_ring_addr.as_u64());
        });

        Ok(())
    }

    fn set_segment_base_addr(&mut self, segment_base_addr: PhysAddr) {
        self
            .registers
            .interrupt_register_set
            .update_volatile_at(0, |r| {
                r.erstba.set(segment_base_addr.as_u64());
            })
    }

    fn init_segment_size(&mut self) {
        self
            .registers
            .interrupt_register_set
            .update_volatile_at(0, |r| {
                // TODO? Segment Size
                r.erstsz.set(1);
            });
    }

    fn set_dequeue_ptr(&mut self, dequeue_ptr: PhysAddr) {
        self.registers.interrupt_register_set.update_volatile_at(0, |r| {
            r.erdp.set_event_ring_dequeue_pointer(dequeue_ptr.as_u64());
        })
    }

    fn interrupt_enable(&mut self) -> KernelResult<()> {
        self.registers.interrupt_register_set.update_volatile_at(0, |r| {
            r.imod.set_interrupt_moderation_interval(4000);
            r.iman.clear_interrupt_pending();
            r.iman.set_interrupt_enable();
        });

        self.registers.operational.usbcmd.update_volatile(|r| {
            r.set_interrupter_enable();
        });

        Ok(())
    }
}
//
//
// #[test_case]
// pub fn should_halted_xhc_controller() {
//     let mut xhc = LibBaseController::try_new(extract_virtual_phys_addr(), 1).unwrap();
//     assert!(xhc.wait_xhc_halted().is_ok())
// }
//
// #[test_case]
// pub fn should_reset_xhc_controller() {
//     let mut xhc = LibBaseController::try_new(extract_virtual_phys_addr(), 1).unwrap();
//     xhc.wait_xhc_halted().unwrap();
//
//     assert!(xhc.reset_controller().is_ok());
// }
//
// #[test_case]
// pub fn should_set_device_context_base_addr() {
//     let mut xhc = LibBaseController::try_new(extract_virtual_phys_addr(), 1).unwrap();
//     xhc.wait_xhc_halted().unwrap();
//     xhc.reset_controller().unwrap();
//     xhc.set_max_slots(1).unwrap();
//
//     let device_context_base: u64 = 0b100000000;
//     assert!(xhc.set_device_context_base_addr(VirtAddr::new(device_context_base)).is_ok());
// }
//
//
// #[test_case]
// pub fn should_set_device_interrupt() {
//     let mut xhc = LibBaseController::try_new(extract_virtual_phys_addr(), 1).unwrap();
//     xhc.wait_xhc_halted().unwrap();
//     xhc.reset_controller().unwrap();
//     xhc.set_max_slots(1).unwrap();
//     let device_context_base = VirtAddr::new(0b100000000);
//     xhc.set_device_context_base_addr(device_context_base).unwrap();
//
//     assert!(xhc.interrupt_enable().is_ok());
//}