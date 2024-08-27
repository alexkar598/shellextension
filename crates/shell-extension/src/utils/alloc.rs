use std::ffi::c_void;
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
