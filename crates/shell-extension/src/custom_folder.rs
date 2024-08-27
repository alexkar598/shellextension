use crate::id_enumerator::EnumIdList;
use crate::utils::{debug_log, ItemIdList};
use crate::{DLL_REF_COUNT, TEST_GUID};
use lazy_static::lazy_static;
use std::cmp;
use std::ffi::c_void;
use std::ops::BitAnd;
use std::sync::atomic::Ordering;
use std::sync::RwLock;
use windows::Win32::Foundation::{
    E_ACCESSDENIED, E_NOTIMPL, E_POINTER, HWND, LPARAM, S_FALSE, S_OK,
};
use windows::Win32::System::Com::{IBindCtx, IPersist_Impl};
use windows::Win32::UI::Shell::Common::{ITEMIDLIST, STRRET, STRRET_OFFSET};
use windows::Win32::UI::Shell::{
    IEnumIDList, IPersistFolder2, IPersistFolder2_Impl, IPersistFolder_Impl, IShellFolder,
    IShellFolder_Impl, SHCONTF_NONFOLDERS, SHGDNF,
};
use windows_core::{implement, GUID, HRESULT, PCWSTR};

#[implement(IPersistFolder2, IShellFolder)]
pub struct CustomFolder {
    location: RwLock<Option<ItemIdList>>,
}

impl CustomFolder {
    pub fn new() -> Self {
        DLL_REF_COUNT.fetch_add(1, Ordering::SeqCst);
        Self {
            location: RwLock::new(None),
        }
    }
}
impl Default for CustomFolder {
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
        *self.location.write().unwrap() = Some(ItemIdList::from(pidl));
        Ok(())
    }
}
impl IPersistFolder2_Impl for CustomFolder_Impl {
    fn GetCurFolder(&self) -> windows_core::Result<*mut ITEMIDLIST> {
        self.location
            .read()
            .unwrap()
            .as_ref()
            .ok_or(S_FALSE)?
            .to_com_ptr()
    }
}
lazy_static! {
    static ref virtual_fs: Vec<ItemIdList> = vec![vec!["Hi!".as_bytes().into()].into()];
}
impl IShellFolder_Impl for CustomFolder_Impl {
    fn ParseDisplayName(
        &self,
        _hwnd: HWND,
        _pbc: Option<&IBindCtx>,
        _pszdisplayname: &PCWSTR,
        _pcheaten: *const u32,
        _ppidl: *mut *mut ITEMIDLIST,
        _pdwattributes: *mut u32,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn EnumObjects(
        &self,
        _hwnd: HWND,
        grfflags: u32,
        ppenumidlist: *mut Option<IEnumIDList>,
    ) -> HRESULT {
        if grfflags.bitand(SHCONTF_NONFOLDERS.0 as u32) == 0 {
            unsafe { ppenumidlist.write(None) };
            return S_FALSE;
        }
        debug_log("Allocing enum");
        let enumerator = EnumIdList::new(&virtual_fs).into();
        unsafe { ppenumidlist.write(Some(enumerator)) };
        S_OK
    }

    fn BindToObject(
        &self,
        _pidl: *const ITEMIDLIST,
        _pbc: Option<&IBindCtx>,
        _riid: *const GUID,
        _ppv: *mut *mut c_void,
    ) -> windows_core::Result<()> {
        Err(E_ACCESSDENIED.into())
    }

    fn BindToStorage(
        &self,
        _pidl: *const ITEMIDLIST,
        _pbc: Option<&IBindCtx>,
        _riid: *const GUID,
        _ppv: *mut *mut c_void,
    ) -> windows_core::Result<()> {
        Err(E_ACCESSDENIED.into())
    }

    fn CompareIDs(
        &self,
        _lparam: LPARAM,
        pidl1: *const ITEMIDLIST,
        pidl2: *const ITEMIDLIST,
    ) -> HRESULT {
        let pidl1 = ItemIdList::from(pidl1);
        let pidl2 = ItemIdList::from(pidl2);
        match pidl1.cmp(&pidl2) {
            cmp::Ordering::Less => HRESULT(0xFFFF),
            cmp::Ordering::Equal => HRESULT(0),
            cmp::Ordering::Greater => HRESULT(1),
        }
    }

    fn CreateViewObject(
        &self,
        _hwndowner: HWND,
        _riid: *const GUID,
        _ppv: *mut *mut c_void,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetAttributesOf(
        &self,
        _cidl: u32,
        _apidl: *const *const ITEMIDLIST,
        _rgfinout: *mut u32,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetUIObjectOf(
        &self,
        _hwndowner: HWND,
        _cidl: u32,
        _apidl: *const *const ITEMIDLIST,
        _riid: *const GUID,
        _rgfreserved: *const u32,
        _ppv: *mut *mut c_void,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetDisplayNameOf(
        &self,
        _pidl: *const ITEMIDLIST,
        _uflags: SHGDNF,
        pname: *mut STRRET,
    ) -> windows_core::Result<()> {
        // let pidl = ItemIdList::from(pidl);
        let pname = unsafe { pname.as_mut() }.ok_or(E_POINTER)?;
        pname.uType = STRRET_OFFSET.0 as u32;
        pname.Anonymous.uOffset = 2;
        Ok(())
    }

    fn SetNameOf(
        &self,
        _hwnd: HWND,
        _pidl: *const ITEMIDLIST,
        _pszname: &PCWSTR,
        _uflags: SHGDNF,
        _ppidlout: *mut *mut ITEMIDLIST,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }
}
