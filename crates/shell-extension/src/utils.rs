mod alloc;
mod hresult;
mod item_id;
mod itemidlist;
mod outrefcast;

pub use alloc::*;
pub use hresult::*;
pub use item_id::*;
pub use itemidlist::*;
pub use outrefcast::*;
use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;
use windows_core::HSTRING;

pub fn debug_log(text: impl Into<HSTRING>) {
    unsafe { OutputDebugStringW(&text.into()) };
}
