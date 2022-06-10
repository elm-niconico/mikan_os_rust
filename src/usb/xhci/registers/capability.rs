use bitfield_struct::{self, bitfield};
use volatile::Volatile;

// BAR 1 BAR 0
// Operatational Register addr -> Cap Base + CAP LENGTH
// Runtime Register addr -> CAP BASE + RTS OFF
// Doorbell Register addr -> CAP BASE + DB OFF
//#[derive(Debug)]
// #[must_use]
// pub struct Capability<M> {
//     /// Capability Registers Length
//     pub cap_length: Volatile<u8>,
//     /// Host Controller Interface Version Number
//     pub hci_version: Volatile<u8>,
//     /// Structural Parameters 1
//     pub hcs_params1: single::ReadWrite<XhcParameters1, M>,
//     /// Structural Parameters 2
//     pub hcsparams2: single::ReadWrite<XhcParameters2, M>,
//     /// Structural Parameters 3
//     pub hcsparams3: single::ReadWrite<StructuralParameters3, M>,
//     /// Capability Parameters 1
//     pub hccparams1: single::ReadWrite<CapabilityParameters1, M>,
//     /// Doorbell Offset
//     pub db_off: Volatile<u32>,
//     /// Runtime Register Space Offset
//     pub rts_off: single::ReadWrite<RuntimeRegisterSpaceOffset, M>,
//     /// Capability Parameters 2
//     pub hccparams2: single::ReadWrite<CapabilityParameters2, M>,
//     /// Virtualization Based Trusted IO Register Space Offset
//     pub vti_os_off: Volatile<u32>,
// }

#[bitfield(u32)]
pub struct XhcParameters1 {
    pub number_of_device_slots: u8,

    pub number_of_interrupts: u16,

    pub number_of_ports: u8,
}

#[bitfield(u32)]
pub struct XhcParameters2 {
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
    pub scrath_pad_is_restore: bool,

    // システムソフトウェアがxHC用に予約するスクラッチパッドバッファの数を示します。
    #[bits(5)]
    pub max_scratch_pad_buffer_low: u8,
}
