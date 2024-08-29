use std::ffi::OsStr;
use std::mem::MaybeUninit;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::UI::Shell::PropertiesSystem::{PSGetPropertyKeyFromName, PROPERTYKEY};
use windows_core::PCWSTR;

pub fn get_property_key_from_name(name: &str) -> windows_core::Result<PROPERTYKEY> {
    let mut key = MaybeUninit::<PROPERTYKEY>::uninit();
    let name = OsStr::new(name).encode_wide().collect::<Vec<_>>();
    unsafe {
        PSGetPropertyKeyFromName(PCWSTR::from_raw(name.as_ptr()), key.as_mut_ptr())
            .map(|_| key.assume_init())
    }
}
