use bitfield_struct::bitfield;

use crate::impl_debug;

#[bitfield(u128)]
pub struct NormalTrb {
    pub data_buffer_pointer: u64,

    #[bits(17)]
    pub trb_transfer_length: u32,

    #[bits(5)]
    pub td_size: u8,

    #[bits(10)]
    pub interrupter_target: u16,

    pub cycle_bit: bool,

    pub evaluate_next_trb: bool,

    pub interrupt_on_short_packet: bool,
    pub no_snoop: bool,
    pub chain_bit: bool,
    pub interrupt_on_completion: bool,
    pub immediate_data: bool,

    #[bits(2)]
    _resolved: u8,

    pub block_event_interrupt: bool,

    #[bits(6)]
    pub trb_type: u8,

    _resolved2: u16,
}

impl_debug! {
    NormalTrb{
        data_buffer_pointer,
        trb_transfer_length,
        td_size,
        interrupter_target,
        cycle_bit,
        evaluate_next_trb,
        interrupt_on_short_packet,
        no_snoop,
        chain_bit,
        interrupt_on_completion,
        immediate_data,
        block_event_interrupt,
        trb_type
    }
}
