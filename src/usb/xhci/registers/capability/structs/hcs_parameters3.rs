use bitfield_struct::bitfield;

use crate::impl_debug_from_methods;


#[bitfield(u32)]
pub struct HcsParameters3 {
    // ルートハブのポートリンク状態（PLS）をU1からU0に移行するための最悪の場合の遅延。すべてのルートハブポートに適用されます。許容値は次のとおりです。
    pub u1_device_exit_latency: u8,
    
    _resolve: u8,
    
    //U2からU0に移行するためのワーストケースのレイテンシーを示します。すべてのルートハブポートに適用されます。許容値は次のとおりです。
    pub u2_device_exit_latency: u16,
}
impl_debug_from_methods! {
    HcsParameters3{
        u1_device_exit_latency,
        u2_device_exit_latency
    }
}
