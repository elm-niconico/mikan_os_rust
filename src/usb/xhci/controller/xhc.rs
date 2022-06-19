use crate::usb::rings::command_ring::CommandRing;
use crate::usb::xhci::device_manager::device_manager::DeviceManager;
use crate::usb::xhci::registers::create_type::RegisterCreate;
use crate::usb::xhci::registers::host_controller_registers::HostControllerRegisters;
use crate::usb::xhci::registers::operational::structs::operational_registers::{OperationalRegisters, XhcResetOperations};
use crate::usb::xhci::registers::volatile::VolatileRegister;


pub struct XhcController {
    xhc_registers: HostControllerRegisters,
    device_manager: DeviceManager,
    command_ring: CommandRing,
}


impl XhcController {
    pub fn initialize(mmio_base_addr: u64, device_max_slots: u8) -> Result<Self, ()> {
        let mut me = Self::new(mmio_base_addr)?;

        me.operations().wait_xhc_halted()?;
        me.operations().reset_controller()?;
        me.set_max_slots(device_max_slots)?;
        me.set_dcbaap()?;
        
        let command_ring_buff_addr = me.command_ring.buffer_addr();
        me.operations().crctl.register_command_ring(command_ring_buff_addr);
        
        me.xhc_registers.runtimes_mut().interrupter_register_set[0].set_enable_interrupt()?;
        me.operations().usb_cmd.set_enable_interrupt()?;
        
        Ok(me)
    }
    
    pub fn run(&mut self) -> Result<(), ()> {

        self.operations().usb_cmd.update_volatile(|cmd| {
            cmd.set_run_stop(true);
        });
        
        while self
            .operations()
            .usb_sts
            .read_volatile()
            .hc_halted()
        {}
        
        if self
            .operations()
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
        let xhc_registers = HostControllerRegisters::new(RegisterCreate::UncheckTransmute, mmio_base)?;
        let device_manager = DeviceManager::new(8);
        let command_ring = CommandRing::new();
        Ok(Self {
            xhc_registers,
            device_manager,
            command_ring,
        })
    }
    
    
    fn operations(&mut self)-> &mut OperationalRegisters{
        self.xhc_registers.operations_mut()
    }
}


trait XhcInitializeOperations {
    fn set_max_slots(&mut self, max_slots: u8) -> Result<(), ()>;
    fn set_dcbaap(&mut self) -> Result<(), ()>;
}


impl XhcInitializeOperations for XhcController {
    fn set_max_slots(&mut self, max_slots: u8) -> Result<(), ()> {
        let cap = self.xhc_registers.capabilities_mut();
        
        let limit_slots = cap
            .hcs_params1
            .read_volatile()
            .number_of_device_slots();
        if max_slots > limit_slots {
            return Err(());
        }
        
        
        let op = self.xhc_registers.operations_mut();
        // max_slotsをキャプチャできないため、updateでは出来ない
        let mut configure = op.configure.read_volatile();
        configure.set_max_device_slots_enabled(max_slots);
        op.configure
          .write_volatile(configure);
        
        if op
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
    
    fn set_dcbaap(&mut self) -> Result<(), ()> {
        let op = self.xhc_registers.operations_mut();
        let mut dcb_aap = op
            .dcbaap
            .read_volatile();
        
        dcb_aap.set_dcbaap(self.device_manager.get_device_context_arr_raw_ptr());
        
        op
            .dcbaap
            .write_volatile(dcb_aap);
        
        let is_setting_ptr = op.dcbaap.read_volatile().dcbaap() != 0;
        
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
pub fn should_xhc_set_max_slots() {
    let mut xhc = XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr()).unwrap();
    
    let res = xhc.set_max_slots(8);
    assert!(res.is_ok())
}


#[test_case]
pub fn should_xhc_set_dcb_base_addr() {
    let mut xhc = XhcController::new(crate::utils::test_fn::extract_virtual_mmio_base_addr()).unwrap();
    
    xhc.set_max_slots(8).unwrap();
    assert!(xhc.set_dcbaap().is_ok());
}


#[test_case]
pub fn should_xhc_initialize() {
    assert!(XhcController::initialize(crate::utils::test_fn::extract_virtual_mmio_base_addr(), 11).is_ok());
}
