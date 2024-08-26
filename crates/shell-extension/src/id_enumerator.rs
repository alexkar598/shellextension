use std::ops::{Deref, Index};
use std::sync::atomic::{AtomicU32, Ordering};
use windows::Win32::Foundation::S_OK;
use windows::Win32::UI::Shell::{IEnumIDList, IEnumIDList_Impl};
use windows::Win32::UI::Shell::Common::ITEMIDLIST;
use windows_core::{implement, HRESULT};

#[implement(IEnumIDList)]
pub struct EnumIdList<'a, T> where T : Index<ITEMIDLIST> {
    index: AtomicU32,
    list: &'a T,
}

impl<'a, T: Index<ITEMIDLIST>> Clone for EnumIdList<'a, T> {
    fn clone(&self) -> Self {
        EnumIdList {
            index: self.index.load(Ordering::Acquire).into(),
            list: self.list
        }
    }
}

impl<'a, T : Index<ITEMIDLIST>> IEnumIDList_Impl for EnumIdList_Impl<'a, T> {
    fn Next(&self, celt: u32, rgelt: *mut *mut ITEMIDLIST, pceltfetched: *mut u32) -> HRESULT {
        todo!()
    }

    fn Skip(&self, celt: u32) -> HRESULT {
        self.index.fetch_add(celt, Ordering::Release);
        S_OK
    }

    fn Reset(&self) -> HRESULT {
        self.index.swap(0, Ordering::Release);
        S_OK
    }

    fn Clone(&self, ppenum: *mut Option<IEnumIDList>) -> HRESULT {
        let cloned = self.deref().clone();
        unsafe { ppenum.write(Some(cloned.into())); }
        S_OK
    }
}
