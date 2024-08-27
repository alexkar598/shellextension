use std::mem::MaybeUninit;
use windows::Win32::Foundation::E_OUTOFMEMORY;
use windows::Win32::System::Com::CoTaskMemAlloc;

pub fn alloc_com_ptr<T>(size: usize) -> windows::core::Result<&'static mut MaybeUninit<T>> {
    unsafe {
        let memory = CoTaskMemAlloc(size).cast::<MaybeUninit<T>>();
        memory.as_mut().ok_or(E_OUTOFMEMORY.into())
    }
} 