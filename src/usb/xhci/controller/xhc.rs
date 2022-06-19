use crate::usb::rings::command_ring::CommandRing;
use crate::usb::xhci::device_manager::device_manager::DeviceManager;
use crate::usb::xhci::registers::capability::create::create_all_registers::ICreateAllCapabilityRegisters;
use crate::usb::xhci::registers::capability::structs::capability_register::CapabilityRegisters;
use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::operational::create::create_all_registers::ICreateAllOperationalRegisters;
use crate::usb::xhci::registers::operational::structs::operational_registers::OperationalRegisters;
use crate::usb::xhci::registers::read_write::volatile::IVolatile;
use crate::usb::xhci::registers::runtime::create::create_runtime_registers::ICreateRuntimeRegisters;
use crate::usb::xhci::registers::runtime::structs::runtime_registers::RuntimeRegisters;


pub struct XhcController {
    capability_register: CapabilityRegisters,
    operational_registers: OperationalRegisters,
    runtime_registers: RuntimeRegisters,
    device_manager: DeviceManager,
    command_ring: CommandRing,
}


impl XhcController {
    pub fn initialize(mmio_base_addr: u64, device_max_slots: u8) -> Result<Self, ()> {
        let mut me = Self::new(mmio_base_addr)?;
        me.wait_usb_halted()?;
        me.reset_controller()?;
        me.set_max_slots(device_max_slots)?;
        me.set_dcb_aap()?;
        
        
        let command_ring_buff_addr = me.command_ring.buffer_addr();
        me.operational_registers.crctl.register_command_ring(command_ring_buff_addr);
        
        me.runtime_registers.interrupter_register_set[0].set_enable_interrupt()?;
        me.operational_registers.usb_cmd.set_enable_interrupt()?;
        
        Ok(me)
    }
    
    pub fn run(&mut self) -> Result<(), ()> {
        self.operational_registers.usb_cmd.update_volatile(|cmd| {
            cmd.set_run_stop(true);
        });
        
        while self
            .operational_registers
            .usb_sts
            .read_volatile()
            .hc_halted()
        {}
        
        if self
            .operational_registers
            .usb_cmd
            .read_volatile()
            .run_stop()
        {
            Ok(())
        } else {
            Err(())
        }
    }
    
    fn new(mmio_base: u64) -> Result<Self, ()> {
        let create = CreateType::UncheckTransmute;
        
        let capability_register = create.new_all_capabilities(mmio_base)?;
        let operational_registers =
            create.new_all_operations(mmio_base, capability_register.cap_length.read_volatile())?;
        let runtime_registers = create.new_runtime_registers(mmio_base, capability_register.rts_off.read_volatile().rts_off())?;
        let command_ring = CommandRing::new();
        Ok(Self {
            capability_register,
            operational_registers,
            runtime_registers,
            command_ring,
            device_manager: DeviceManager::new(8),
        })
    }
}


pub trait IXhcResetOperations {
    fn wait_usb_halted(&mut self) -> Result<(), ()>;
    fn reset_controller(&mut self) -> Result<(), ()>;
}


impl IXhcResetOperations for XhcController {
    fn wait_usb_halted(&mut self) -> Result<(), ()> {
        self.operational_registers.usb_cmd.update_volatile(|cmd| {
            cmd.set_interrupt_enable(false);
            cmd.set_host_system_error_enable(false);
            cmd.set_enable_wrap_event(false);
        });
        let is_not_halted = |me: &XhcController| {
            !me
                .operational_registers
                .usb_sts
                .read_volatile()
                .hc_halted()
        };
        
        if is_not_halted(self) {
            self.operational_registers.usb_cmd.update_volatile(|cmd| {
                cmd.set_run_stop(false);
            });
        }
        
        while is_not_halted(self) {}
        
        let is_stop_controller = !is_not_halted(self) && !self.operational_registers.usb_cmd.read_volatile().run_stop();
        if is_stop_controller {
            Ok(())
        } else {
            Err(())
        }
    }
    
    fn reset_controller(&mut self) -> Result<(), ()> {
        let read_cmd = |me: &XhcController| me.operational_registers.usb_cmd.read_volatile();
        let read_sts = |me: &XhcController| me.operational_registers.usb_sts.read_volatile();
        self.operational_registers.usb_cmd.update_volatile(|cmd| {
            cmd.set_host_controller_reset(true);
        });
        
        while read_cmd(self).host_controller_reset() {}
        
        while read_sts(self).controller_not_ready() {}
        
        if (!read_cmd(self).host_controller_reset()) && (!read_sts(self).controller_not_ready()) {
            Ok(())
        } else {
            Err(())
        }
    }
}


trait IXhcInitializeOperations {
    fn set_max_slots(&mut self, max_slots: u8) -> Result<(), ()>;
    fn set_dcb_aap(&mut self) -> Result<(), ()>;
}


impl IXhcInitializeOperations for XhcController {
    fn set_max_slots(&mut self, max_slots: u8) -> Result<(), ()> {
        let limit_slots = self
            .capability_register
            .hcs_params1
            .read_volatile()
            .number_of_device_slots();
        if max_slots > limit_slots {
            return Err(());
        }
        
        // max_slotsをキャプチャできないため、updateでは出来ない
        let mut configure = self.operational_registers.configure.read_volatile();
        configure.set_max_device_slots_enabled(max_slots);
        self.operational_registers
            .configure
            .write_volatile(configure);
        
        if self
            .operational_registers
            .configure
            .read_volatile()
            .max_device_slots_enabled()
            == max_slots
        {
            Ok(())
        } else {
            Err(())
        }
    }
    
    fn set_dcb_aap(&mut self) -> Result<(), ()> {
        let mut dcb_aap = self
            .operational_registers
            .dcbaap
            .read_volatile();
        
        dcb_aap.set_dcbaap(self.device_manager.get_device_context_arr_raw_ptr());
        
        self.operational_registers
            .dcbaap
            .write_volatile(dcb_aap);
        
        let is_setting_ptr = self.operational_registers.dcbaap.read_volatile().dcbaap() != 0;
        
        if is_setting_ptr {
            Ok(())
        } else {
            Err(())
        }
    }
}


#[test_case]
pub fn should_new_xhc() {
    let xhc = XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr());
    
    assert!(xhc.is_ok());
}


#[test_case]
pub fn should_run_xhc() {
    let mut xhc = XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr()).unwrap();
    let run_res = xhc.run();
    assert!(run_res.is_ok())
}


#[test_case]
pub fn should_wait_hc_halted() {
    let mut xhc = XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr()).unwrap();
    
    let halted_res = xhc.wait_usb_halted();
    assert!(halted_res.is_ok())
}


#[test_case]
pub fn should_xhc_reset() {
    let mut xhc = XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr()).unwrap();
    
    let reset_res = xhc.reset_controller();
    assert!(reset_res.is_ok())
}


#[test_case]
pub fn should_xhc_set_max_slots() {
    let mut xhc = XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr()).unwrap();
    
    let res = xhc.set_max_slots(8);
    assert!(res.is_ok())
}


#[test_case]
pub fn should_xhc_set_dcb_base_addr() {
    let mut xhc = XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr()).unwrap();
    
    xhc.set_max_slots(8).unwrap();
    assert!(xhc.set_dcb_aap().is_ok());
}


#[test_case]
pub fn should_xhc_initialize() {
    assert!(XhcController::initialize(crate::utils::test_fn::extract_virtual_mmio_base_addr(), 11).is_ok());
}
