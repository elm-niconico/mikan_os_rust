use crate::usb::xhci::registers::read_write::volatile::Volatile;


pub enum CreateType {
    // 生ポインタからレジスタの構造体に強制的に解釈
    // 安全ではありません
    UncheckTransmute
}


pub type CreateRegisterResult<T> = Result<Volatile<T>, ()>;
