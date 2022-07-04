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

// TODO Option::Noneからエラーを作成したい