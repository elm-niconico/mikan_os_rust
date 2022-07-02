use core::slice;

use crate::serial_println;
use crate::usb_my::xhci::controller::port::Port;
use crate::usb_my::xhci::device_manager::device_manager::DeviceManager;
use crate::usb_my::xhci::registers::create_type::RegisterCreate;
use crate::usb_my::xhci::registers::host_controller_registers::HostControllerRegisters;
use crate::usb_my::xhci::registers::operational::structs::operational_registers::{
    OperationalRegisters, XhcResetOperations,
};
use crate::usb_my::xhci::registers::operational::structs::port_register_set::PortRegisterSet;
use crate::usb_my::xhci::registers::volatile::VolatileRegister;
use crate::usb_my::xhci::rings::command_ring::CommandRing;
use crate::usb_my::xhci::rings::event_ring::EventRing;
use crate::utils::error::CommonResult;

pub struct XhcController {
    xhc_registers: HostControllerRegisters,
    device_manager: DeviceManager,
    command_ring: CommandRing,
    event_ring: EventRing,
    mmio_virt_addr: u64,
}

impl XhcController {
    pub fn initialize(
        mmio_base_addr: u64,
        physical_memory_offset: u64,
        device_max_slots: u8,
    ) -> CommonResult<XhcController> {
        let res = XhcController::new(mmio_base_addr, physical_memory_offset);
        if let Err(error) = res {
            return Err(error);
        }

        let mut me = res.unwrap();
        me.operations().wait_xhc_halted()?;

        me.operations().reset_controller()?;

        me.set_max_slots(device_max_slots)?;

        // let hcs_params2 = me.xhc_registers.capabilities_mut().hcs_params2.read();
        // let max_scratchpad_buffers = hcs_params2.max_scratchpad_buffers_high() << 5 | hcs_params2.max_scratch_pad_buffer_low();
        // serial_println!("max {:?}", hcs_params2);
        // serial_println!("event {:?}", me.xhc_registers.capabilities_mut().hcc_params1.read());
        me.set_dcbaap()?;

        let command_ring_buff_addr = me.command_ring.buffer_addr();

        me.operations()
            .crctl
            .register_command_ring(command_ring_buff_addr);

        me.xhc_registers
            .runtimes_mut()
            .interrupter_register_set
            .primary()
            .set_enable_interrupt()?;

        me.operations().usb_cmd.set_enable_interrupt()?;

        Ok(me)
    }

    pub fn run(&mut self) -> CommonResult<()> {
        self.operations().usb_cmd.update(|cmd| {
            cmd.set_run_stop(true);
        });

        while self.operations().usb_sts.read().hc_halted() {}

        if self.operations().usb_cmd.read().run_stop() {
            Ok(())
        } else {
            Err("Failed Run Controller")
        }
    }

    pub fn new(mmio_virt_addr: u64, physical_offset: u64) -> CommonResult<XhcController> {
        let create = RegisterCreate::UncheckTransmute;

        let xhc_registers = HostControllerRegisters::new(create, mmio_virt_addr);

        let mut xhc_registers = xhc_registers.unwrap();

        let device_manager = DeviceManager::new(8);

        let command_ring = CommandRing::new();

        let event_ring = EventRing::new(xhc_registers.runtimes_mut(), physical_offset);
        Ok(Self {
            xhc_registers,
            device_manager,
            command_ring,
            event_ring,
            mmio_virt_addr,
        })
    }

    pub fn max_pots(&mut self) -> u8 {
        self.xhc_registers
            .capabilities_mut()
            .hcs_params1
            .read()
            .number_of_ports()
    }

    pub fn process_event(&mut self) {
        let primary = self
            .xhc_registers
            .runtimes_mut()
            .interrupter_register_set
            .primary();
        if !self.event_ring.has_front(&primary) {
            return;
        }

        let event_trb = self.event_ring.front_trb(&primary);
        if event_trb.trb_type() != 0 {
            serial_println!("event trb type {:?}", event_trb.trb_type());
        }

        self.event_ring.pop(
            &mut self
                .xhc_registers
                .runtimes_mut()
                .interrupter_register_set
                .primary(),
        );
    }

    pub fn reset_all_ports(&mut self) -> CommonResult<()> {
        for n in 0..self.max_pots() + 1 {
            let mut port = self.port_at(n as usize);
            if port.is_current_connect() {
                port.reset()?
            }
            serial_println!("RESET {:?}", port);
        }
        Ok(())
    }
    pub fn port_at(&mut self, index: usize) -> Port {
        let port_sets = (self.port_register_sets());
        Port::new(&port_sets[index])
    }

    fn port_register_sets(&mut self) -> &mut [PortRegisterSet] {
        let cap_len: u64 = self
            .xhc_registers
            .capabilities_mut()
            .cap_length
            .read()
            .cap_len() as u64;
        let port_register_set_addr = self.mmio_virt_addr + cap_len + 0x400;

        unsafe {
            slice::from_raw_parts_mut(
                port_register_set_addr as *mut PortRegisterSet,
                (self.max_pots() + 1) as usize,
            )
        }
    }
    fn operations(&mut self) -> &mut OperationalRegisters {
        self.xhc_registers.operations_mut()
    }
}

trait XhcInitializeOperations {
    fn set_max_slots(&mut self, max_slots: u8) -> CommonResult<()>;
    fn set_dcbaap(&mut self) -> CommonResult<()>;
}

impl XhcInitializeOperations for XhcController {
    fn set_max_slots(&mut self, max_slots: u8) -> CommonResult<()> {
        let cap = self.xhc_registers.capabilities_mut();

        let limit_slots = cap.hcs_params1.read().number_of_device_slots();
        if max_slots > limit_slots {
            return Err("Argument Max Slots Its Over Limit Slots");
        }

        let op = self.xhc_registers.operations_mut();

        let mut configure = op.configure.read();
        configure.set_max_device_slots_enabled(max_slots);
        op.configure.write(configure);

        if op.configure.read().max_device_slots_enabled() == max_slots {
            Ok(())
        } else {
            Err("Failed Set Slot Max Device")
        }
    }

    fn set_dcbaap(&mut self) -> CommonResult<()> {
        let op = self.xhc_registers.operations_mut();
        let mut dcb_aap = op.dcbaap.read();

        dcb_aap.set_dcbaap(self.device_manager.get_device_context_arr_raw_ptr());

        op.dcbaap.write(dcb_aap);

        let is_setting_ptr = op.dcbaap.read().dcbaap() != 0;

        if is_setting_ptr {
            Ok(())
        } else {
            Err("Failed Setting Device Context Base Address Array Pointer")
        }
    }
}

// #[test_case]
// pub fn should_new_xhc() {
//     let xhc = XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr(), 1);
//
//     assert!(xhc.is_ok());
// }
//
//
// #[test_case]
// pub fn should_run_xhc() {
//     let mut xhc =
//         XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr(), 1).unwrap();
//     let run_res = xhc.run();
//     assert!(run_res.is_ok())
// }
//
//
// #[test_case]
// pub fn should_xhc_set_max_slots() {
//     let mut xhc =
//         XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr(), 1).unwrap();
//
//     let res = xhc.set_max_slots(8);
//     assert!(res.is_ok())
// }
//
//
// #[test_case]
// pub fn should_xhc_set_dcb_base_addr() {
//     let mut xhc =
//         XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr(), 1).unwrap();
//
//     xhc.set_max_slots(8).unwrap();
//     assert!(xhc.set_dcbaap().is_ok());
// }
//
//
// #[test_case]
// pub fn should_xhc_initialize() {
//     let is_success = XhcController::initialize(
//         crate::utils::test_fn::extract_virtual_mmio_base_addr(),
//         1,
//         8,
//     )
//         .is_ok();
//     assert!(is_success);
// }
//
//
// #[test_case]
// pub fn should_read_max_pots() {
//     let mut xhc = XhcController::initialize(
//         crate::utils::test_fn::extract_virtual_mmio_base_addr(),
//         1,
//         8,
//     )
//         .unwrap();
//     assert_ne!(0, xhc.max_pots());
// }
// #[test_case]
// pub fn should_reset_all_ports() {
//     let mut xhc = XhcController::initialize(
//         crate::utils::test_fn::extract_virtual_mmio_base_addr(),
//         1,
//         8,
//     )
//         .unwrap();
//     assert!(xhc.reset_all_ports().is_ok());
// }