use bitfield_struct::{self, bitfield};

use crate::{impl_debug, impl_debug_only_fields, impl_deref_from_type};

// BAR 1 BAR 0
// Operatational Register addr -> Cap Base + CAP LENGTH
// Runtime Register addr -> CAP BASE + RTS OFF
// Doorbell Register addr -> CAP BASE + DB OFF

#[repr(C)]
#[allow(dead_code)]
pub struct CapabilityRegister {
    /// Capability Registers Length
    pub cap_length: CapLength,
    /// Host Controller Interface Version Number
    pub hci_version: VciVersion,
    /// Structural Parameters 1
    pub hcs_params1: XhcParameters1,
    /// Structural Parameters 2
    pub hcs_params2: XhcParameters2,
    /// Structural Parameters 3
    pub hcs_params3: XhcParameters3,
    /// Capability Parameters 1
    pub hcc_params1: XhcParameters1,
    /// Doorbell Offset
    pub db_off: DbOff,
    /// Runtime Register Space Offset
    pub rts_off: RuntimeRegisterSpaceOffset,
    /// Capability Parameters 2
    pub hcc_params2: RuntimeRegisterSpaceOffset,
    /// Virtualization Based Trusted IO Register Space Offset
    pub vti_os_off: XhcParameters2,
}

impl_debug_only_fields! {
    CapabilityRegister{
        cap_length,
        hci_version,
        hcs_params1,
        hcs_params2,
        hcs_params3,
        hcc_params1,
        db_off,
        rts_off,
        hcc_params2,
        vti_os_off
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapLength(u8);
impl_deref_from_type!(CapLength, u8);
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VciVersion(u8);
impl_deref_from_type!(VciVersion, u8);

#[bitfield(u32)]
pub struct XhcParameters1 {
    pub number_of_device_slots: u8,
    
    pub number_of_interrupts: u16,
    
    pub number_of_ports: u8,
}
impl_debug! {
    XhcParameters1{
        number_of_device_slots,
        number_of_interrupts,
        number_of_ports
    }
}

#[bitfield(u32)]
pub struct XhcParameters2 {
    #[bits(4)]
    pub iso_chronous_scheduling_threshold: u8,
    
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
impl_debug! {
    XhcParameters2{
        iso_chronous_scheduling_threshold,
        event_ring_segment_max,
        max_scratchpad_buffers_high,
        scratch_pad_is_restore,
        max_scratch_pad_buffer_low
    }
}

#[bitfield(u32)]
pub struct XhcParameters3 {
    // ルートハブのポートリンク状態（PLS）をU1からU0に移行するための最悪の場合の遅延。すべてのルートハブポートに適用されます。許容値は次のとおりです。
    pub u1_device_exit_latency: u8,
    
    _resolve: u8,
    
    //U2からU0に移行するためのワーストケースのレイテンシーを示します。すべてのルートハブポートに適用されます。許容値は次のとおりです。
    pub u2_device_exit_latency: u16,
}
impl_debug! {
    XhcParameters3{
        u1_device_exit_latency,
        u2_device_exit_latency
    }
}

#[bitfield(u32)]
pub struct DbOff {
    #[bits(2)]
    _resolved: u8,
    
    #[bits(30)]
    // DoorbellRegisterのアドレス = CAP_BASE + doorbel_array_offset
    pub doorbell_array_offset: u32,
}
impl_debug! {
    DbOff{
        doorbell_array_offset
    }
}

#[bitfield(u32)]
pub struct RuntimeRegisterSpaceOffset {
    #[bits(5)]
    _resolve: u8,
    #[bits(27)]
    pub runtime_register_space_offset: u32,
}
impl_debug! {
    RuntimeRegisterSpaceOffset{
        runtime_register_space_offset
    }
}
