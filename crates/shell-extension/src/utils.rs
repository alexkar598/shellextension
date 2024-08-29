mod alloc;
mod hresult;
mod item_id;
mod itemidlist;
mod outrefcast;
mod property;

pub use alloc::*;
pub use hresult::*;
pub use item_id::*;
pub use itemidlist::*;
pub use outrefcast::*;
pub use property::*;
use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;
use windows_core::HSTRING;

pub fn debug_log(text: impl Into<HSTRING>) {
    unsafe { OutputDebugStringW(&text.into()) };
}
pub fn not_implemented<T>(message: &str, code: windows_core::HRESULT) -> windows::core::Result<T> {
    debug_log(format!("Not implemented: {message}"));
    Err(code.into())
}
