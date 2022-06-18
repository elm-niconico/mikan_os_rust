use core::fmt::Debug;

use crate::usb::xhci::registers::read_write::volatile::Volatile;


#[allow(dead_code)]
pub enum CreateType {
    // 生ポインタからレジスタの構造体に強制的に解釈
    // 安全ではありません
    UncheckTransmute
}


pub type CreateRegisterResult<T: Debug> = Result<Volatile<T>, ()>;
