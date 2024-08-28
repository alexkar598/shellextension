use std::ffi::{c_void, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::str::EncodeUtf16;
use windows::Win32::Foundation::E_OUTOFMEMORY;
use windows::Win32::System::Com::CoTaskMemAlloc;

pub fn alloc_com_ptr(size: usize) -> windows::core::Result<*mut c_void> {
    unsafe {
        let memory = CoTaskMemAlloc(size);
        match memory.is_null() {
            true => Err(E_OUTOFMEMORY.into()),
            false => Ok(memory),
        }
    }
}

pub trait ToComPtr<T> {
    fn to_com_ptr(self) -> windows::core::Result<(*mut T, usize)>;
}

impl<T> ToComPtr<T> for &str {
    fn to_com_ptr(self) -> windows_core::Result<(*mut T, usize)> {
        self.encode_utf16().to_com_ptr()
    }
}
impl<T> ToComPtr<T> for &OsStr {
    fn to_com_ptr(self) -> windows_core::Result<(*mut T, usize)> {
        self.encode_wide().to_com_ptr()
    }
}

impl<T> ToComPtr<T> for EncodeUtf16<'_> {
    fn to_com_ptr(self) -> windows_core::Result<(*mut T, usize)> {
        let string = self.collect::<Vec<_>>();
        let utf16_length = string.len();
        let byte_length = (utf16_length + 1) * size_of::<u16>();
        let memory = alloc_com_ptr(byte_length)?.cast::<u16>();
        unsafe {
            memory.copy_from_nonoverlapping(string.as_ptr(), utf16_length);
            memory.wrapping_add(utf16_length).write(0);
        };
        let memory = memory.cast::<T>();
        Ok((memory, byte_length))
    }
}
