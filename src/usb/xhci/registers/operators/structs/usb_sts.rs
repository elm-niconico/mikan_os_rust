use bitfield_struct::bitfield;
use crate::impl_debug_from_methods;


#[allow(dead_code)]
#[bitfield(u32)]
pub struct UsbStsRegister {
    pub is_hc_halted: bool,
    _reserve: bool,
    pub host_system_error: bool,
    pub event_interrupt: bool,
    pub port_change_detect: bool,
    
    #[bits(3)]
    _reserve2: u8,
    
    pub save_state_status: bool,
    
    pub restore_state_status: bool,
    pub save_restore_error: bool,
    
    // デフォルト true
    pub controller_not_ready: bool,
    
    pub host_controller_error: bool,
    
    #[bits(19)]
    _reserve3: u32,
}
impl_debug_from_methods!{
    UsbStsRegister{
        is_hc_halted,
        host_system_error,
        port_change_detect,
        save_state_status,
        restore_state_status,
        save_restore_error,
        controller_not_ready,
        host_controller_error
    }
}
