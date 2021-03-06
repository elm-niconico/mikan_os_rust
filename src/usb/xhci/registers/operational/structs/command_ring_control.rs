use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u64)]
pub struct CommandRingControlRegister {
    // RCS このビットはコマンドリングが動いている間常に1(True)と書き込まれます
    pub ring_cycle_state: bool,
    
    // デフォルト0
    // 1を書き込むと現在のコマンドを実行してからリングの動作を中止します。
    pub command_stop: bool,
    
    // デフォルト 0
    // CommandRing == 0ならばこのフラグへの書き込みは無視されます。
    // このビットに1を書き込むとコマンドリングの動作を中止させるコマンドを即時作成します。
    pub command_abort: bool,
    
    // デフォルト 0
    // (R/S)Bitが1の時かつドアベルレジスタがホストらコントローラーコマンドによって書き込まれている間、値は１です。
    // (R/s)かCommandAbortに1が書き込まれた場合にリセットされます。
    pub command_ring_running: bool,
    
    #[bits(2)]
    _reserve: u8,
    
    #[bits(58)]
    pub command_ring_pointer: u64,
}

impl_debug_bit_fields! {
    CommandRingControlRegister{
        ring_cycle_state,
        command_stop,
        command_abort,
        command_ring_running,
        command_ring_pointer
    }
}
