use crate::usb::xhci::registers::volatile::Volatile;


pub enum RegisterCreate {
    // 生ポインタからレジスタの構造体に強制的に解釈
    // 安全ではありません
    UncheckTransmute
}


pub type CreateRegisterResult<T> = Result<Volatile<T>, ()>;
