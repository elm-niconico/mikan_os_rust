#[derive(Debug)]
pub struct DeviceContextErrInfo<T> {
    pub expect: T,
    pub actual: T,
}