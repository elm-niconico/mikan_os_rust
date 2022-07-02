use bitfield_struct::bitfield;
use crate::impl_debug_bit_fields;


#[bitfield(u32)]
pub struct HccParams2{
    pub u3_entry_capability: bool,
    pub configure_endpoint_command: bool,
    pub force_save_context_capability: bool,
    pub compliance_transition_capability: bool,
    pub large_esit_payload_capability: bool,
    pub configuration_information_capability: bool,
    pub extended_tbc_capability: bool,
    pub extended_tbc_trb_status: bool,
    pub extended_property_capability: bool,
    pub virtualization_based_trusted: bool,
    
    #[bits(22)]
    _reserve: u32
}
impl_debug_bit_fields!{
    HccParams2{
        u3_entry_capability,
        configure_endpoint_command,
        force_save_context_capability,
        compliance_transition_capability,
        large_esit_payload_capability,
        configuration_information_capability,
        extended_tbc_capability,
        extended_tbc_trb_status,
        extended_property_capability,
        virtualization_based_trusted
    }
}
