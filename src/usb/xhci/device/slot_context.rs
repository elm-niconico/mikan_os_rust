use core::fmt::Debug;

use bitfield_struct::bitfield;

use crate::{impl_debug_from_methods, impl_debug_only_fields};


pub struct SlotContext {
    pub offset_0: SlotContextOffset0,
    pub offset_1: SlotContextOffset1,
    pub offset_2: SlotContextOffset2,
    pub offset_3: SlotContextOffset3,
}

impl_debug_only_fields! {
    SlotContext{
        offset_0,
        offset_1,
        offset_2,
        offset_3
    }
}


#[bitfield(u32)]
pub struct SlotContextOffset0 {
    #[bits(20)]
    pub route_string: u32,
    
    #[bits(4)]
    pub speed: u8,
    
    _reserve: bool,
    
    pub mtt: bool,
    pub hub: bool,
    
    #[bits(5)]
    pub context_entries: u8,
}



impl_debug_from_methods! {
    SlotContextOffset0{
        route_string,
        speed,
        mtt,
        hub,
        context_entries
    }
}


#[bitfield(u32)]
pub struct SlotContextOffset1 {
    pub max_exit_latency: u16,
    
    pub root_hub_port_number: u8,
    
    pub number_of_ports: u8,
}

impl_debug_from_methods! {
    SlotContextOffset1{
        max_exit_latency,
        root_hub_port_number,
        number_of_ports
    }
}



#[bitfield(u32)]
pub struct SlotContextOffset2 {
    pub parent_hub_slot_id: u8,
    
    pub parent_port_number: u8,
    
    #[bits(2)]
    pub tt_think_time: u8,
    
    #[bits(4)]
    _reserve2: u8,
    
    #[bits(10)]
    pub interrupter_target: u16,
}
impl_debug_from_methods! {
    SlotContextOffset2{
        parent_hub_slot_id,
        parent_port_number,
        tt_think_time,
        interrupter_target
    }
}


#[bitfield(u32)]
pub struct SlotContextOffset3 {
    pub usb_device_addr: u8,
    
    #[bits(19)]
    _reserve: u32,
    
    #[bits(5)]
    pub slot_state: u8,
}
impl_debug_from_methods! {
    SlotContextOffset3{
        usb_device_addr,
        slot_state
    }
}
