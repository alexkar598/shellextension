use bytemuck::cast_slice;
use std::ffi::{c_void, OsStr};
use std::os::windows::ffi::OsStrExt;
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

impl<T> ToComPtr<T> for &[u8] {
    fn to_com_ptr(self) -> windows_core::Result<(*mut T, usize)> {
        let utf16_length = self.len();
        let byte_length = (utf16_length + 1) * size_of::<u16>();
        let memory = alloc_com_ptr(byte_length)?.cast::<u8>();
        unsafe {
            memory.copy_from_nonoverlapping(self.as_ptr(), utf16_length);
            memory.wrapping_add(utf16_length).write(0);
        };
        let memory = memory.cast::<T>();
        Ok((memory, byte_length))
    }
}
impl<T> ToComPtr<T> for &OsStr {
    fn to_com_ptr(self) -> windows_core::Result<(*mut T, usize)> {
        let mut string = self.encode_wide().collect::<Vec<_>>();
        string.reserve_exact(2);
        string.push(0);
        string.push(0);
        let string = cast_slice(string.as_slice());
        string.to_com_ptr()
    }
}
