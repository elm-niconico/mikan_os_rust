use bitfield_struct::bitfield;

use crate::impl_debug_from_methods;


#[bitfield(u32)]
pub struct HccParams1 {
    pub bit_64_address_capability: bool,
    pub bw_negotiation_capability: bool,
    pub context_size: bool,
    pub port_power_control: bool,
    pub port_indicators: bool,
    pub light_hc_reset_capability: bool,
    pub latency_tolerance_messaging_capability: bool,
    pub no_secondary_sid_support: bool,
    pub parse_all_event_data: bool,
    pub stopped_short_packet_capability: bool,
    pub stopped_edtla_capability: bool,
    pub contiguous_frame_id_capability: bool,
    
    #[bits(4)]
    pub max_primary_stream_array_size: u8,
    pub xhci_extended_capabilities_pointer: u16,
}


impl_debug_from_methods! {
    HccParams1{
        bit_64_address_capability,
        bw_negotiation_capability,
        context_size,
        port_power_control,
        light_hc_reset_capability,
        latency_tolerance_messaging_capability,
        no_secondary_sid_support,
        parse_all_event_data,
        stopped_short_packet_capability,
        stopped_edtla_capability,
        contiguous_frame_id_capability,
        max_primary_stream_array_size,
        xhci_extended_capabilities_pointer
    }
}
