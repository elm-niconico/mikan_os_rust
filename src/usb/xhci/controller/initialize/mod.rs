use x86_64::VirtAddr;

use crate::error::KernelResult;
use crate::usb::xhci::device_manager::device_manager::DeviceContextAddr;
use crate::usb::xhci::rings::command_ring::CommandRingAddress;
use crate::usb::xhci::rings::event_ring::EventRingAddress;

mod lib_base_initializer;


pub fn init_xhci<T>(xhc: &mut T, max_slots: u8) -> KernelResult<()>
    where T:
    XhcInitializer +
    EventRingAddress +
    CommandRingAddress +
    DeviceContextAddr
{
    xhc.wait_xhc_halted()?;
    xhc.reset_controller()?;
    xhc.set_max_slots(max_slots)?;
    xhc.set_device_context_base_addr(xhc.device_context_base_addr())?;
    xhc.set_segment_base_addr(xhc.segment_tbl_base_addr());
    xhc.init_segment_size();
    xhc.set_dequeue_ptr(xhc.dequeue_ptr_addr());
    xhc.register_command_ring(xhc.command_ring_base_addr())?;
    xhc.interrupt_enable()?;

    Ok(())
}


/// Xhcの初期化処理を行いました。
pub trait XhcInitializer {
    /// XHCの動作を停止させます。
    /// 停止させるまでブロッキングを行います
    fn wait_xhc_halted(&mut self) -> KernelResult<()>;

    /// XHCの設定をリセットさせます。
    fn reset_controller(&mut self) -> KernelResult<()>;

    /// デバイスの最大接続数を設定させます。
    fn set_max_slots(&mut self, device_max_slots: u8) -> KernelResult<()>;

    /// デバイスコンテキストのベースアドレスをコンフィギュレーションに設定します。
    /// デバイスコンテキストのバッファーは実装側が作成する必要があります。
    fn set_device_context_base_addr(&mut self, device_context_addr: VirtAddr) -> KernelResult<()>;


    /// コマンドリングのベースアドレスをコンフィギュレーションに設定します。
    /// コマンドリングのバッファーは実装側が作成する必要があります。
    fn register_command_ring(&mut self, command_ring_addr: VirtAddr) -> KernelResult<()>;


    ///TODO COMMENT
    fn set_segment_base_addr(&mut self, segment_base_addr: VirtAddr);


    // TODO COMMENT Segment Tbl Size Register
    fn init_segment_size(&mut self);


    /// コマンドリングのデータバッファーの先頭アドレスを設定します。
    fn set_dequeue_ptr(&mut self, dequeue_ptr: VirtAddr);


    /// 割り込みの許可を知らせるコマンドを追加します。
    fn interrupt_enable(&mut self) -> KernelResult<()>;
}