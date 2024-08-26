use std::ffi::c_void;
use std::ops::BitAnd;
use std::sync::atomic::Ordering;
use std::sync::RwLock;
use windows::Win32::Foundation::{E_NOTIMPL, HWND, LPARAM, S_FALSE};
use windows::Win32::System::Com::{IBindCtx, IPersist_Impl};
use windows::Win32::UI::Shell::Common::{ITEMIDLIST, SHCOLSTATE, SHELLDETAILS, STRRET};
use windows::Win32::UI::Shell::{IEnumExtraSearch, IEnumIDList, IPersistFolder2, IPersistFolder2_Impl, IPersistFolder_Impl, IShellFolder2, IShellFolder2_Impl, IShellFolder_Impl, SHCONTF_NONFOLDERS, SHGDNF};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_DEFBUTTON1};
use windows_core::{implement, w, GUID, HRESULT, PCWSTR, VARIANT};
use crate::{DLL_REF_COUNT, TEST_GUID};

#[implement(IPersistFolder2, IShellFolder2)]
pub struct CustomFolder {
    location: RwLock<Option<ITEMIDLIST>>
}

impl CustomFolder {
    pub fn new() -> Self {
        DLL_REF_COUNT.fetch_add(1, Ordering::SeqCst);
        Self {
            location: RwLock::new(None)
        }
    }
}
impl Default for CustomFolder{
    fn default() -> Self {
        CustomFolder::new()
    }
}

impl Drop for CustomFolder {
    fn drop(&mut self) {
        DLL_REF_COUNT.fetch_sub(1, Ordering::SeqCst);
    }
}

impl IPersist_Impl for CustomFolder_Impl {
    fn GetClassID(&self) -> windows_core::Result<GUID> {
        Ok(TEST_GUID)
    }
}
impl IPersistFolder_Impl for CustomFolder_Impl {
    fn Initialize(&self, pidl: *const ITEMIDLIST) -> windows_core::Result<()> {
        unsafe {
            MessageBoxW(HWND::default(), w!("We had printf debugging 50 years ago. Somehow, we can't even achieve that and we have to resort to using fucking message boxes because microsoft decided in all their wisdom that explorer.exe should restart itself every single time no matter what combinasion of arcane registry keys and command line arguments you pass in because microsoft hates you, hates me and hates everything sane and we can't have anything remotely nice like starting up a child process with a debugger attached."), w!("Debugging"), MB_DEFBUTTON1);
        }
        *self.location.write().unwrap() = Some(unsafe {*pidl});
        Ok(())
    }
}
impl IPersistFolder2_Impl for CustomFolder_Impl {
    fn GetCurFolder(&self) -> windows_core::Result<*mut ITEMIDLIST> {
        self.location.read().unwrap().map(|x| (&x as *const ITEMIDLIST).cast_mut()).ok_or(S_FALSE.into())
    }
}
impl IShellFolder_Impl for CustomFolder_Impl {
    fn ParseDisplayName(&self, _hwnd: HWND, _pbc: Option<&IBindCtx>, _pszdisplayname: &PCWSTR, _pcheaten: *const u32, _ppidl: *mut *mut ITEMIDLIST, _pdwattributes: *mut u32) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn EnumObjects(&self, _hwnd: HWND, grfflags: u32, ppenumidlist: *mut Option<IEnumIDList>) -> HRESULT {
        if grfflags.bitand(SHCONTF_NONFOLDERS.0 as u32) > 0 {
            unsafe {ppenumidlist.write(None)};
            return S_FALSE
        }
        E_NOTIMPL
    }

    fn BindToObject(&self, _pidl: *const ITEMIDLIST, _pbc: Option<&IBindCtx>, _riid: *const GUID, _ppv: *mut *mut c_void) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn BindToStorage(&self, _pidl: *const ITEMIDLIST, _pbc: Option<&IBindCtx>, _riid: *const GUID, _ppv: *mut *mut c_void) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn CompareIDs(&self, _lparam: LPARAM, _pidl1: *const ITEMIDLIST, _pidl2: *const ITEMIDLIST) -> HRESULT {
        E_NOTIMPL
    }

    fn CreateViewObject(&self, _hwndowner: HWND, _riid: *const GUID, _ppv: *mut *mut c_void) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetAttributesOf(&self, _cidl: u32, _apidl: *const *const ITEMIDLIST, _rgfinout: *mut u32) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetUIObjectOf(&self, _hwndowner: HWND, _cidl: u32, _apidl: *const *const ITEMIDLIST, _riid: *const GUID, _rgfreserved: *const u32, _ppv: *mut *mut c_void) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetDisplayNameOf(&self, _pidl: *const ITEMIDLIST, _uflags: SHGDNF, _pname: *mut STRRET) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn SetNameOf(&self, _hwnd: HWND, _pidl: *const ITEMIDLIST, _pszname: &PCWSTR, _uflags: SHGDNF, _ppidlout: *mut *mut ITEMIDLIST) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }
}
impl IShellFolder2_Impl for CustomFolder_Impl {
    fn GetDefaultSearchGUID(&self) -> windows_core::Result<GUID> {
        Err(E_NOTIMPL.into())
    }

    fn EnumSearches(&self) -> windows_core::Result<IEnumExtraSearch> {
        Err(E_NOTIMPL.into())
    }

    fn GetDefaultColumn(&self, _dwres: u32, _psort: *mut u32, _pdisplay: *mut u32) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetDefaultColumnState(&self, _icolumn: u32) -> windows_core::Result<SHCOLSTATE> {
        Err(E_NOTIMPL.into())
    }

    fn GetDetailsEx(&self, _pidl: *const ITEMIDLIST, _pscid: *const windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<VARIANT> {
        Err(E_NOTIMPL.into())
    }

    fn GetDetailsOf(&self, _pidl: *const ITEMIDLIST, _icolumn: u32, _psd: *mut SHELLDETAILS) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn MapColumnToSCID(&self, _icolumn: u32, _pscid: *mut windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }
}