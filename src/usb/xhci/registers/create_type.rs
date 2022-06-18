use core::fmt::Debug;
use crate::usb::xhci::registers::register_info::RegisterInfo;


#[allow(dead_code)]
pub enum CreateType {
    // 生ポインタからレジスタの構造体に強制的に解釈
    // 安全ではありません
    UncheckTransmute,
    
    // 生ポインタからレジスタの構造体にキャスト
    // ビットフィールドの値をチェックします
    TransmuteWithCheck,
}



pub type CreateRegisterResult<T: Debug> = Result<RegisterInfo<T>, ()>;
