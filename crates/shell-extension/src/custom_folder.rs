use crate::id_enumerator::EnumIdList;
use crate::utils::{
    debug_log, get_property_key_from_name, not_implemented, BoundedDeref, BoundedDerefMut,
    ItemIdList, ToComPtr,
};
use crate::{DLL_REF_COUNT, TEST_GUID, TEST_PROPERTY_GUID};
use lazy_static::lazy_static;
use std::cmp;
use std::ffi::{c_void, OsString};
use std::mem::ManuallyDrop;
use std::ops::BitAnd;
use std::sync::atomic::Ordering;
use std::sync::RwLock;
use windows::Win32::Foundation::{E_FAIL, E_NOTIMPL, E_POINTER, HWND, LPARAM, S_FALSE, S_OK};
use windows::Win32::System::Com::StructuredStorage::PID_FIRST_USABLE;
use windows::Win32::System::Com::{IBindCtx, IPersist_Impl};
use windows::Win32::UI::Controls::LVCFMT_LEFT;
use windows::Win32::UI::Shell::Common::{
    ITEMIDLIST, SHCOLSTATE, SHCOLSTATE_ONBYDEFAULT, SHCOLSTATE_TYPE_STR, SHELLDETAILS, STRRET,
    STRRET_WSTR,
};
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;
use windows::Win32::UI::Shell::{
    IEnumExtraSearch, IEnumIDList, IPersistFolder, IPersistFolder2, IPersistFolder2_Impl,
    IPersistFolder_Impl, IShellFolder, IShellFolder2, IShellFolder2_Impl, IShellFolder_Impl,
    SHCreateShellFolderView, SFV_CREATE, SHCONTF_FOLDERS, SHCONTF_NONFOLDERS, SHGDNF,
};
use windows_core::{implement, IUnknownImpl, Interface, GUID, HRESULT, PCWSTR, PWSTR, VARIANT};

#[implement(IPersistFolder, IPersistFolder2, IShellFolder, IShellFolder2)]
pub struct CustomFolder {
    location: RwLock<Option<ItemIdList<'static>>>,
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
        pidl.with_ref(|pidl| {
            let pidl = ItemIdList::from(pidl).into_owned();
            debug_log(format!("CustomFolder.Initialize: pidl:{pidl:?}"));
            *self.location.write().unwrap() = Some(pidl);
            Ok(())
        })
    }
}
impl IPersistFolder2_Impl for CustomFolder_Impl {
    fn GetCurFolder(&self) -> windows_core::Result<*mut ITEMIDLIST> {
        debug_log("CustomFolder.GetCurFolder");
        let location = self.location.read().unwrap();
        let location = location.as_ref().ok_or(S_FALSE)?;
        debug_log(format!(
            "CustomFolder.GetCurFolder/ret: location:{location:?}"
        ));
        Ok(location.to_com_ptr()?.0)
    }
}
lazy_static! {
    static ref virtual_fs: Vec<ItemIdList<'static>> = vec![
        vec!["Hi!"].into(),
        vec!["Meow, "].into(),
        vec!["comment"].into(),
        vec!["ça"].into(),
        vec!["va"].into(),
        vec!["?"].into(),
    ];
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
        not_implemented("ParseDisplayName", E_NOTIMPL)
    }

    fn EnumObjects(
        &self,
        _hwnd: HWND,
        grfflags: u32,
        ppenumidlist: *mut Option<IEnumIDList>,
    ) -> HRESULT {
        debug_log(format!(
            "EnumObjects: flags:{grfflags} folders:{} nonfolders:{}",
            grfflags.bitand(SHCONTF_FOLDERS.0 as u32) > 0,
            grfflags.bitand(SHCONTF_NONFOLDERS.0 as u32) > 0
        ));
        if grfflags.bitand(SHCONTF_FOLDERS.0 as u32) == 0 {
            unsafe { ppenumidlist.write(None) };
            return S_FALSE;
        }
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
        not_implemented("BindToObject", E_NOTIMPL)
    }

    fn BindToStorage(
        &self,
        _pidl: *const ITEMIDLIST,
        _pbc: Option<&IBindCtx>,
        _riid: *const GUID,
        _ppv: *mut *mut c_void,
    ) -> windows_core::Result<()> {
        not_implemented("BindToStorage", E_NOTIMPL)
    }

    fn CompareIDs(
        &self,
        _lparam: LPARAM,
        pidl1: *const ITEMIDLIST,
        pidl2: *const ITEMIDLIST,
    ) -> HRESULT {
        pidl1
            .with_ref(|pidl1| {
                pidl2.with_ref(|pidl2| {
                    let pidl1 = ItemIdList::from(pidl1);
                    let pidl2 = ItemIdList::from(pidl2);
                    debug_log(format!(
                        "CustomFolder.CompareIDs: pidl1:{pidl1:?} pidl2:{pidl2:?}"
                    ));
                    let result = match pidl1.cmp(&pidl2) {
                        cmp::Ordering::Less => HRESULT(0xFFFF),
                        cmp::Ordering::Equal => HRESULT(0),
                        cmp::Ordering::Greater => HRESULT(1),
                    };
                    Ok(result)
                })
            })
            .into()
    }

    fn CreateViewObject(
        &self,
        _hwndowner: HWND,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            debug_log(format!(
                "CustomFolder.CreateViewObject: _hwndowner:{_hwndowner:?} riid:{:?} ppv:{ppv:?}",
                *riid
            ));
            let options = SFV_CREATE {
                cbSize: size_of::<SFV_CREATE>() as u32,
                pshf: ManuallyDrop::new(Some(self.to_interface::<IShellFolder>())),
                psvOuter: ManuallyDrop::new(None),
                psfvcb: ManuallyDrop::new(None),
            };
            let view = SHCreateShellFolderView(&options)?;
            let view = view.query(riid, ppv).ok();
            debug_log(format!("CreateViewObject/ret: {view:?}, {:?}", *riid));
            view
        }
    }

    fn GetAttributesOf(
        &self,
        _cidl: u32,
        _apidl: *const *const ITEMIDLIST,
        _rgfinout: *mut u32,
    ) -> windows_core::Result<()> {
        not_implemented("GetAttributesOf", E_NOTIMPL)
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
        not_implemented("GetUIObjectOf", E_NOTIMPL)
    }

    fn GetDisplayNameOf(
        &self,
        pidl: *const ITEMIDLIST,
        _uflags: SHGDNF,
        pname: *mut STRRET,
    ) -> windows_core::Result<()> {
        pidl.with_ref(|pidl| {
            let pidl = ItemIdList::from(pidl);
            if pidl.len() != 1 {
                return Err(E_FAIL.into());
            }
            let pidl = pidl[0].as_ref();
            debug_log(format!(
                "CustomFolder.GetDisplayNameOf: pidl:{pidl:?} flags:{_uflags:?} pagename:{pname:?}",
            ));
            pname.with_mut_ref(|pname| {
                pname.uType = STRRET_WSTR.0 as u32;
                pname.Anonymous.pOleStr = PWSTR::from_raw(pidl.to_com_ptr()?.0);
                Ok(())
            })
        })
    }

    fn SetNameOf(
        &self,
        _hwnd: HWND,
        _pidl: *const ITEMIDLIST,
        _pszname: &PCWSTR,
        _uflags: SHGDNF,
        _ppidlout: *mut *mut ITEMIDLIST,
    ) -> windows_core::Result<()> {
        not_implemented("SetNameOf", E_NOTIMPL)
    }
}

lazy_static! {
    static ref virtual_fs_columns: Vec<(OsString, PROPERTYKEY, SHCOLSTATE)> = vec![
        (
            OsString::from("Name"),
            get_property_key_from_name("System.ItemNameDisplay").unwrap(),
            SHCOLSTATE(SHCOLSTATE_TYPE_STR.0 | SHCOLSTATE_ONBYDEFAULT.0)
        ),
        (
            OsString::from("Test"),
            PROPERTYKEY {
                fmtid: TEST_PROPERTY_GUID,
                pid: PID_FIRST_USABLE
            },
            SHCOLSTATE(SHCOLSTATE_TYPE_STR.0 | SHCOLSTATE_ONBYDEFAULT.0)
        ),
    ];
}
impl IShellFolder2_Impl for CustomFolder_Impl {
    fn GetDefaultSearchGUID(&self) -> windows_core::Result<GUID> {
        not_implemented("GetDefaultSearchGUID", E_NOTIMPL)
    }

    fn EnumSearches(&self) -> windows_core::Result<IEnumExtraSearch> {
        not_implemented("EnumSearches", E_NOTIMPL)
    }

    fn GetDefaultColumn(
        &self,
        dwres: u32,
        psort: *mut u32,
        pdisplay: *mut u32,
    ) -> windows_core::Result<()> {
        not_implemented("GetDefaultColumn", E_NOTIMPL)
    }

    fn GetDefaultColumnState(&self, icolumn: u32) -> windows_core::Result<SHCOLSTATE> {
        debug_log(format!("CustomFolder.GetDefaultColumnState: col:{icolumn}"));
        if let Some(column) = virtual_fs_columns.get(icolumn as usize) {
            Ok(column.2)
        } else {
            Err(E_FAIL.into())
        }
    }

    fn GetDetailsEx(
        &self,
        pidl: *const ITEMIDLIST,
        pscid: *const PROPERTYKEY,
    ) -> windows_core::Result<VARIANT> {
        not_implemented("GetDetailsEx", E_NOTIMPL)
    }

    fn GetDetailsOf(
        &self,
        pidl: *const ITEMIDLIST,
        icolumn: u32,
        psd: *mut SHELLDETAILS,
    ) -> windows_core::Result<()> {
        pidl.with_ref(|pidl| {
            debug_log(format!(
                "CustomFolder.GetDetailsOf: pid:{:?} col:{icolumn}",
                ItemIdList::from(pidl)
            ));

            if let Some(column) = virtual_fs_columns.get(icolumn as usize) {
                let column = &column.0;
                let (column, size): (*mut u16, _) = column.to_com_ptr()?;
                psd.with_mut_ref(|psd| {
                    psd.fmt = LVCFMT_LEFT.0;
                    psd.cxChar = size as i32;
                    psd.str.uType = STRRET_WSTR.0 as u32;
                    psd.str.Anonymous.pOleStr = PWSTR::from_raw(column);
                    Ok(())
                })
            } else {
                Err(E_FAIL.into())
            }
        })
    }

    fn MapColumnToSCID(&self, icolumn: u32, pscid: *mut PROPERTYKEY) -> windows_core::Result<()> {
        debug_log(format!(
            "CustomFolder.MapColumnToSCID: col:{icolumn} pscid:{pscid:?}"
        ));

        let result = if let Some(column) = virtual_fs_columns.get(icolumn as usize) {
            pscid.with_mut_ref(|x| {
                x.clone_from(&column.1);
                Ok(())
            })?;
            Ok(())
        } else {
            Err(E_FAIL.into())
        };
        debug_log(format!(
            "CustomFolder.MapColumnToSCID/ret: col:{icolumn} {result:?}"
        ));
        result
    }
}
