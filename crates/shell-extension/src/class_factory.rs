use std::ffi::c_void;
use std::sync::atomic::Ordering;
use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Com::{IClassFactory_Impl, IClassFactory, CoLockObjectExternal};
use windows_core::{implement, IUnknown, Interface, GUID};
use crate::{DLL_REF_COUNT};
use crate::custom_folder::CustomFolder;

#[implement(IClassFactory)]
pub struct ClassFactory {
    _private: ()
}

impl IClassFactory_Impl for ClassFactory_Impl {
    fn CreateInstance(&self, _punkouter: Option<&IUnknown>, riid: *const GUID, ppvobject: *mut *mut c_void) -> windows::core::Result<()> {
        let instance = CustomFolder::new();
        unsafe { IUnknown::from(instance).query(riid, ppvobject) }.ok()
    }

    fn LockServer(&self, flock: BOOL) -> windows::core::Result<()> {
        unsafe { CoLockObjectExternal(&self.cast::<IUnknown>()?, flock, true) }
    }
}

impl ClassFactory {
    pub fn new() -> Self {
        DLL_REF_COUNT.fetch_add(1, Ordering::SeqCst);
        Self {
            _private: ()
        }
    }
}
impl Default for ClassFactory {
    fn default() -> Self {
        ClassFactory::new()
    }
}
impl Drop for ClassFactory {
    fn drop(&mut self) {
        DLL_REF_COUNT.fetch_sub(1, Ordering::SeqCst);
    }
}
