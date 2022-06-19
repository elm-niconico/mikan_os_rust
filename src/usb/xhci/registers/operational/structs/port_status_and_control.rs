use bitfield_struct::bitfield;
use crate::impl_debug_bit_filed;


#[bitfield(u32)]
pub struct PortStatusControlRegister {
    pub current_connect_status: bool,
    
    // ソフトウェアによって1が書き込まれる可能性がある
    // ポートの状態が変わっている間はこのビットを変更してはいけない
    // PED
    pub port_enabled_disabled: bool,
    
    _reserve: bool,
    
    // 1は過電流(?)状態
    pub over_current_active: bool,
    
    // デフォルト 0
    // 1はポートのリセットシグナルをあらわすっぽい
    pub port_reset: bool,
    
    // このフィールドはポートの電源管理に使用され、現在のリンク状態を反映します
    // ソフトウェアによってリンクの状態を書き換えるのに使う？
    // PLS
    // TODO よくわからん
    #[bits(4)]
    pub port_link_state: u8,
    
    // PP
    pub port_power: bool,
    
    #[bits(4)]
    pub protocol_speed: u8,
    
    // 0 = Port Indicators Are Off
    // 1 = Amber
    // 2 = Green
    // 3 = Undefined
    #[bits(2)]
    pub port_indicator_control: u8,
    
    // PLSフィールドの書き込みの有効性を示す
    // このビットは読み込むと必ず0になる？
    pub port_link_state_write_strobe: bool,
    
    // XHCコントローラーから設定される?
    pub connect_status_change: bool,
    
    // ソフトウェアによってport_powerが0に設定されている場合、このフラグは設定されません。
    // PEC
    pub port_enabled_disabled_change: bool,
    
    // 0 = No Cahnge
    // 1 = 再起動完了
    // ソフトウェアによって明示的にPPまたはPEDに0を書き込んで再起動した場合は1をセットされません。
    pub warm_port_reset_change: bool,
    
    // OCC
    pub over_current_change: bool,
    
    // PRC
    // port_resetの値が変更されたら1が書き込まれます。
    // 強制的にPPやPEDを書き換えたらセットされません
    pub port_reset_change: bool,
    
    // extend_xhci.pdfの412ページ参照
    pub port_link_state_change: bool,
    
    // CEC
    // ポートリンク設定に失敗したときに1が設定されます
    // USB2では予約領域で使われることはない
    pub port_config_error_change: bool,
    
    // TODO よくわからん attach status
    pub cold_attach_status: bool,
    
    // デバイスの接続をシステム起動イベントとして認識するようにする。
    pub wake_on_connect_enable: bool,
    
    pub wake_on_disconnect_enable: bool,
    
    // 過電流の検知をシステムイベントとして認識するようにする?
    // TODO よくわからん
    pub wake_on_over_current_enable: bool,
    
    #[bits(2)]
    _reserve2: u8,
    
    // デバイスが取り外されていたら(false)0
    pub device_removable: bool,
    
    pub warm_port_reset: bool
}
impl_debug_bit_filed!{
    PortStatusControlRegister{
        current_connect_status,
        port_enabled_disabled,
        over_current_active,
        port_reset,
        port_link_state,
        protocol_speed,
        port_indicator_control,
        warm_port_reset_change,
        over_current_change,
        port_reset_change,
        port_config_error_change,
        cold_attach_status,
        wake_on_connect_enable,
        wake_on_disconnect_enable,
        wake_on_over_current_enable,
        device_removable,
        warm_port_reset
    }
}
