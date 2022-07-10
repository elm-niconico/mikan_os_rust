use core::ptr;

use x86_64::{PhysAddr, VirtAddr};
use xhci::Registers;

use crate::error::KernelResult;
use crate::serial_println;
use crate::usb::xhci::device_manager::device_manager::{DeviceContextAddr, DeviceManager};
use crate::usb::xhci::mapper::XhcMapper;
use crate::usb::xhci::rings::command_ring::{CommandRing, CommandRingAddress};
use crate::usb::xhci::rings::event_ring::{EventRing, EventRingAddress};
use crate::usb::xhci::trb::trb_base::TrbBase;

pub struct LibBaseController {
    pub registers: Registers<XhcMapper>,
    pub command_ring: CommandRing,
    pub event_ring: EventRing,
    pub device_manager: DeviceManager,
}

unsafe impl Send for LibBaseController {}

unsafe impl Sync for LibBaseController {}

impl LibBaseController {
    pub fn try_new(xhc_mmio_base: PhysAddr, device_max_slots: u8) -> KernelResult<Self> {
        serial_println!("Try New Xhc");
        let mapper = XhcMapper::new();

        let registers = unsafe { Registers::new(xhc_mmio_base.as_u64() as usize, mapper) };
        serial_println!("Create Register");
        let command_ring = CommandRing::new();
        let event_ring = unsafe { EventRing::new() };
        let device_manager = DeviceManager::try_new(device_max_slots)?;

        Ok(Self {
            registers,
            command_ring,
            event_ring,
            device_manager,
        })
    }

    pub fn run(&mut self) {
        self.registers.operational.usbcmd.update_volatile(|r| {
            r.set_run_stop();
        });

        while self.registers.operational.usbsts.read_volatile().hc_halted() {};
    }

    pub fn process_event(&mut self) {
        let erdp = self.registers.interrupt_register_set.read_volatile_at(0).erdp.event_ring_dequeue_pointer();
        let trb_base = unsafe { ptr::read_volatile(erdp as *const TrbBase) };
        if !trb_base.cycle_bit() {
            serial_println!("Parameter {:?}", trb_base.parameter());
            return;
        }


        self.log_usb_sts();
        self.log_usb_cmd();

        serial_println!("TRB BASE {:#?}", trb_base);
    }

    pub fn ports(&self) {
        for i in 0..self.registers.capability.hcsparams1.read_volatile().number_of_ports() {
            let port = self.registers.port_register_set.read_volatile_at(i as usize);
            let is_connect = port.portsc.current_connect_status();
            serial_println!("Port {} \n {:#?}", i, port);
        }
    }


    #[allow(unused)]
    pub fn log_usb_sts(&self) {
        serial_println!("{:#?}", self.registers.operational.usbsts.read_volatile());
    }

    #[allow(unused)]
    pub fn log_usb_cmd(&self) {
        serial_println!("{:#?}", self.registers.operational.usbcmd.read_volatile());
    }

    #[allow(unused)]
    pub fn is_run_command_ring(&self) -> bool {
        self.registers.operational.crcr.read_volatile().command_ring_running()
    }
}


impl EventRingAddress for LibBaseController {
    fn dequeue_ptr_addr(&self) -> VirtAddr {
        self.event_ring.dequeue_ptr_addr()
    }

    fn segment_tbl_base_addr(&self) -> VirtAddr {
        self.event_ring.segment_tbl_base_addr()
    }
}

impl CommandRingAddress for LibBaseController {
    fn command_ring_base_addr(&self) -> VirtAddr {
        self.command_ring.command_ring_base_addr()
    }
}


impl DeviceContextAddr for LibBaseController {
    fn device_context_base_addr(&self) -> VirtAddr {
        self.device_manager.device_context_base_addr()
    }
}