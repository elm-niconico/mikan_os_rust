use bitfield_struct::bitfield;

#[bitfield(u32)]
pub struct UsbCmd {
    // 1にすることで動作の開始をスケジュールします。
    // USB STSのHC HALTEDが1にななるまで待つ必要があります
    pub is_run_stop: bool,

    // ソフトウェアによって設定されます
    // 1を書き込むと、タイマー、カウンター、状態などが初期値にリセットされます。
    pub is_host_contoroller_reset: bool,

    // true(1)の時ホストシステムからの割り込みを許可します。
    // デフォルト false
    pub is_intterupt_enable: bool,

    // TODO out-of-bandの説明
    pub is_host_system_error_enable: bool,
    #[bits(3)]
    _reserve: u8,

    // デフォルト false
    // ソフトウェアはこのビットが0の時LightHostControllerがリセットされ再初期化できる状態だと解釈します。
    // 1の時はまだリセットされていないと解釈します。
    pub is_light_host_controller_reset: bool,

    // デフォルト false
    // ソフトウェアによってtrueが書き込まれた場合に,HC HALTEDは1が書き込まれ、その後、Xhcのない状態が保存されます。
    pub controller_save_state: bool,


    // デフォルト false
    // ソフトウェアによってtrueが書き込まれると以前の状態が復元されます。
    pub controller_restore_state: bool,


    // 
    pub is_enable_wrap_event: bool,
    pub is_enable_u3_find_index_stop: bool,
    pub is_stopped_short_packet_enable: bool,
    pub is_cem_enable: bool,

    #[bits(18)]
    _reserve2: u32,
}
