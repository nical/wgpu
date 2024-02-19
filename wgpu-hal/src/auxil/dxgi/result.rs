use std::borrow::Cow;

use crate::DeviceError;
use winapi::shared::winerror;

pub(crate) trait HResult<O> {
    fn into_result(self) -> Result<O, Cow<'static, str>>;
    fn into_device_result(self, description: &str) -> Result<O, DeviceError>;
}
impl HResult<()> for i32 {
    fn into_result(self) -> Result<(), Cow<'static, str>> {
        if self >= 0 {
            return Ok(());
        }
        let description = match self {
            winerror::E_UNEXPECTED => "unexpected",
            winerror::E_NOTIMPL => "not implemented",
            winerror::E_OUTOFMEMORY => "out of memory",
            winerror::E_INVALIDARG => "invalid argument",
            winerror::DXGI_ERROR_DEVICE_RESET => "device reset",
            winerror::DXGI_ERROR_DEVICE_REMOVED => "device removed",
            _ => return Err(Cow::Owned(format!("0x{:X}", self as u32))),
        };
        Err(Cow::Borrowed(description))
    }
    fn into_device_result(self, description: &str) -> Result<(), DeviceError> {
        self.into_result().map_err(|err| {
            log::error!("{} failed: {}", description, err);
            match self {
                winerror::E_OUTOFMEMORY => DeviceError::OutOfMemory,
                winerror::DXGI_ERROR_DEVICE_REMOVED | winerror::DXGI_ERROR_DEVICE_RESET => {
                    DeviceError::Lost
                }
                _ => DeviceError::Unknown,
            }
        })
    }
}

impl<T> HResult<T> for (T, i32) {
    fn into_result(self) -> Result<T, Cow<'static, str>> {
        self.1.into_result().map(|()| self.0)
    }
    fn into_device_result(self, description: &str) -> Result<T, DeviceError> {
        self.1.into_device_result(description).map(|()| self.0)
    }
}
