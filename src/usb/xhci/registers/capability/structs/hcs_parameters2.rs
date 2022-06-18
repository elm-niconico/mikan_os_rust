use crate::impl_debug_from_methods;
use bitfield_struct::bitfield;

#[bitfield(u32)]
pub struct HcsParameters2 {
    #[bits(4)]
    pub isochronous_scheduling_threshold: u8,
    
    // このフィールドは、イベントリングセグメントテーブルのベースサイズレジスタでサポートされる最大値を決定します。
    #[bits(4)]
    pub event_ring_segment_max: u8,
    
    #[bits(13)]
    _resolved: u16,
    
    #[bits(5)]
    pub max_scratchpad_buffers_high: u8,
    
    // 0 =スクラッチパッドのバッファスペースが解放され、電源イベント間で再割り当てされる可能性があることを示します。
    // 1 = xHCが、電源イベント間でスクラッチパッドバッファースペースの整合性を維持する必要があることを示します。
    pub scratch_pad_is_restore: bool,
    
    // システムソフトウェアがxHC用に予約するスクラッチパッドバッファの数を示します。
    #[bits(5)]
    pub max_scratch_pad_buffer_low: u8,
}
impl_debug_from_methods! {
    HcsParameters2{
        isochronous_scheduling_threshold,
        event_ring_segment_max,
        max_scratchpad_buffers_high,
        scratch_pad_is_restore,
        max_scratch_pad_buffer_low
    }
}
