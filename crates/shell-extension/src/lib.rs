#![feature(try_trait_v2)]
#![feature(ptr_metadata)]

mod class_factory;
mod constants;
mod custom_folder;
mod utils;
mod id_enumerator;

pub use constants::*;

use crate::class_factory::ClassFactory;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use windows::core::GUID;
use windows::Win32::Foundation::{CLASS_E_CLASSNOTAVAILABLE, E_POINTER, S_FALSE, S_OK};
use windows_core::{IUnknown, Interface, OutRef};
use crate::utils::{OutRefExtension, HRESULT};

lazy_static! {
    /// The number of references to this module (DLL)
    static ref DLL_REF_COUNT: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
}

/*
HRESULT DllGetClassObject(
  [in]  REFCLSID rclsid,
  [in]  REFIID   riid,
  [out] LPVOID   *ppv
);
 */
#[no_mangle]
extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    factory: OutRef<IUnknown>,
) -> HRESULT {
    let rclsid = unsafe {rclsid.as_ref()}.ok_or(E_POINTER)?;
    match *rclsid {
        TEST_GUID => 
            unsafe {IUnknown::from(ClassFactory::new()).query(riid, factory.into_ptr())}.into(),
        _ => {
            factory.write(None).err().map_or(CLASS_E_CLASSNOTAVAILABLE, |x| x.into()).into()
        }
    }
}


/*
HRESULT DllCanUnloadNow(
);
 */
#[no_mangle]
extern "system" fn DllCanUnloadNow(
) -> HRESULT {
    match DLL_REF_COUNT.load(Ordering::SeqCst) {
        0 => S_OK,
        _ => S_FALSE
    }.into()
}