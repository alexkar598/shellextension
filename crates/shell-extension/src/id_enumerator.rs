use crate::utils::ItemIdList;
use crate::DLL_REF_COUNT;
use std::ops::Deref;
use std::sync::atomic::{AtomicU32, Ordering};
use windows::Win32::Foundation::{S_FALSE, S_OK};
use windows::Win32::UI::Shell::Common::ITEMIDLIST;
use windows::Win32::UI::Shell::{IEnumIDList, IEnumIDList_Impl};
use windows_core::{implement, HRESULT};

#[implement(IEnumIDList)]
pub struct EnumIdList<'a> {
    index: AtomicU32,
    list: &'a [ItemIdList],
}
impl<'a> EnumIdList<'a> {
    pub fn new(list: &'a [ItemIdList]) -> Self {
        DLL_REF_COUNT.fetch_add(1, Ordering::SeqCst);
        Self {
            index: AtomicU32::new(0),
            list,
        }
    }
}
impl Drop for EnumIdList<'_> {
    fn drop(&mut self) {
        DLL_REF_COUNT.fetch_sub(1, Ordering::SeqCst);
    }
}
impl Clone for EnumIdList<'_> {
    fn clone(&self) -> Self {
        EnumIdList {
            index: self.index.load(Ordering::Acquire).into(),
            ..Self::new(self.list)
        }
    }
}

impl IEnumIDList_Impl for EnumIdList_Impl<'_> {
    fn Next(&self, celt: u32, output: *mut *mut ITEMIDLIST, pceltfetched: *mut u32) -> HRESULT {
        let mut fetched = 0;
        for i in 0..celt as usize {
            if let Some(item) = self
                .list
                .get(self.index.load(Ordering::Acquire) as usize + i)
            {
                if let Ok(ptr) = item.to_com_ptr() {
                    unsafe { output.wrapping_add(i).write(ptr) };
                    fetched += 1;
                    continue;
                }
            }
            break;
        }
        self.index.fetch_add(fetched, Ordering::Release);
        if !pceltfetched.is_null() {
            unsafe {
                pceltfetched.write(fetched);
            }
        }
        match celt == fetched {
            true => S_OK,
            false => S_FALSE,
        }
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
        unsafe {
            ppenum.write(Some(cloned.into()));
        }
        S_OK
    }
}
