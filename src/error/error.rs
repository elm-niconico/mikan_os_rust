use alloc::string::String;
use core::alloc::LayoutError;

use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::page::AddressNotAligned;
use x86_64::structures::paging::Size4KiB;

/// std::errorのような構造体を定義します
/// ?演算子(Question Mark Operator)をより汎用的なものにするのが目的です。
///
///
/// # Examples
///
/// ``` rust
///
/// fn read_file() -> Result<(), Error>{
///     let file = File::open("dummy.text")?;
///     let read = file.read([])?;
///     OK(())
/// }
///
/// ```
#[derive(Debug)]
pub enum KernelError {
    MapToError(MapToError<Size4KiB>),
    AddressNotAligned(AddressNotAligned),
    None,
    Empty,
    FailedHcHalted,
    FailedHcReset,
    OverFlowDeviceMaxSlots,
    FailedSetDeviceContextBase(DeviceContextErrInfo),
    LayoutError(LayoutError)
}


#[derive(Debug)]
pub struct DeviceContextErrInfo {
    pub expect: u64,
    pub actual: u64,
}

impl From<MapToError<Size4KiB>> for KernelError {
    fn from(e: MapToError<Size4KiB>) -> Self {
        Self::MapToError(e)
    }
}

impl From<AddressNotAligned> for KernelError {
    fn from(e: AddressNotAligned) -> Self {
        KernelError::AddressNotAligned(e)
    }
}

impl From<()> for KernelError {
    fn from(_: ()) -> Self {
        Self::Empty
    }
}

impl From<String> for KernelError {
    fn from(_: String) -> Self {
        Self::None
    }
}

impl From<LayoutError> for KernelError {
    fn from(e: LayoutError) -> Self {
        Self::LayoutError(e)
    }
}

// TODO Option::Noneからエラーを作成したい