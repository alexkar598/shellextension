use std::ops::FromResidual;

#[allow(clippy::upper_case_acronyms)]
#[repr(transparent)]
pub struct HRESULT(pub windows::core::HRESULT);
impl FromResidual<Result<std::convert::Infallible, windows::core::HRESULT>> for HRESULT {
    fn from_residual(residual: Result<std::convert::Infallible, windows::core::HRESULT>) -> Self {
        HRESULT(residual.unwrap_err())
    }
}
impl FromResidual<Result<std::convert::Infallible, windows::core::Error>> for HRESULT {
    fn from_residual(residual: Result<std::convert::Infallible, windows::core::Error>) -> Self {
        HRESULT(residual.into())
    }
}
impl From<windows::core::HRESULT> for HRESULT {
    fn from(value: windows_core::HRESULT) -> Self {
        HRESULT(value)
    }
}
impl From<windows::core::Error> for HRESULT {
    fn from(value: windows_core::Error) -> Self {
        HRESULT(value.into())
    }
}