use crate::impl_debug_bit_fields;
use bitfield_struct::bitfield;

// #[bitfield]
// pub struct PortRegisterSet {
//     pub port_sc: PortSc,
//     pub data1: u32,
//     pub data2: u64,
// }

#[bitfield(u128)]
pub struct PortRegisterSet {
    pub current_connect_status: bool,

    pub port_enabled_disabled: bool,
    _reserve: bool,
    pub over_current_active: bool,
    pub port_reset: bool,

    #[bits(4)]
    pub port_link_state: u8,

    pub port_power: bool,

    #[bits(4)]
    pub port_speed: u8,

    #[bits(2)]
    pub port_indicator_control: u8,
    pub port_link_state_write_strobe: bool,
    pub connect_status_change: bool,
    pub port_enabled_disabled_change: bool,
    pub warm_port_reset_change: bool,
    pub over_current_change: bool,
    pub port_reset_change: bool,
    pub port_link_state_change: bool,
    pub port_config_error_change: bool,
    pub cold_attach_status: bool,
    pub wake_on_connect_enable: bool,
    pub wake_on_disconnect_enable: bool,
    pub wake_on_over_current_enable: bool,
    #[bits(2)]
    _reserve2: u8,
    pub device_removable: bool,
    pub warm_port_reset: bool,
    _reserve2: u32,
    _reserve3: u64,
}

impl_debug_bit_fields! {
    PortRegisterSet{
        current_connect_status,
        port_enabled_disabled,
        port_link_state,
        over_current_active,
        port_reset,
        connect_status_change
    }
}
