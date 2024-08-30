use crate::utils::item_id::ItemId;
use crate::utils::{alloc_com_ptr, ToComPtr};
use bytemuck::checked::cast_slice;
use std::borrow::Cow;
use std::borrow::Cow::Owned;
use std::ffi::OsString;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem::transmute;
use std::ops::Deref;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use windows::Win32::UI::Shell::Common::ITEMIDLIST;
use windows_core::imp::E_POINTER;

#[repr(transparent)]
#[derive(PartialEq, PartialOrd)]
pub struct ItemIdList<'a>(pub Vec<Cow<'a, [u8]>>, PhantomData<&'a ()>);
impl<'a> ItemIdList<'a> {
    pub fn into_owned(self) -> ItemIdList<'static> {
        let owned = self
            .0
            .iter()
            .map(|x| Owned(x.clone().into_owned()))
            .collect();
        unsafe { transmute::<ItemIdList<'a>, ItemIdList<'static>>(Self(owned, Default::default())) }
    }
}

impl Debug for ItemIdList<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let e = self
            .0
            .iter()
            .map(|x| {
                x.deref()
                    .iter()
                    .copied()
                    .filter(|&x| x != 0)
                    .collect::<Vec<_>>()
            })
            .map(|x| String::from_utf8_lossy(x.as_slice()).into_owned())
            .collect::<Vec<_>>();
        e.fmt(f)
    }
}

impl ToComPtr<ITEMIDLIST> for &ItemIdList<'_> {
    fn to_com_ptr(self) -> windows::core::Result<(*mut ITEMIDLIST, usize)> {
        let total_size = self.0.iter().map(|x| x.len() + 2).sum::<usize>() + 2;
        let memory = alloc_com_ptr(total_size)?;
        let mut next = memory.cast::<u8>();

        unsafe {
            for item_id in &self.0 {
                let size = item_id.len();
                let item_id = item_id.as_ref().as_ptr();
                next.cast::<u16>().write_unaligned(size as u16 + 2);
                next = next.wrapping_byte_add(2);
                next.copy_from_nonoverlapping(item_id, size);
                next = next.wrapping_byte_add(size);
            }
            next.cast::<u16>().write_unaligned(0u16);
        }
        Ok((memory.cast(), total_size))
    }
}

impl<'a> From<&'a ITEMIDLIST> for ItemIdList<'a> {
    fn from(value: &'a ITEMIDLIST) -> ItemIdList<'a> {
        let mut result = vec![];
        let mut next = ptr::from_ref(value).cast::<u16>();
        loop {
            unsafe {
                let length = next.read_unaligned() as usize;
                if length == 0 {
                    break;
                }

                let entry = ptr::from_raw_parts::<ItemId>(next, length);
                let entry = entry.as_ref().unwrap();
                let entry = Cow::from(&entry.content);
                result.push(entry);
                next = next.wrapping_byte_add(length);
            }
        }
        Self(result, Default::default())
    }
}

impl From<Vec<&str>> for ItemIdList<'_> {
    ///Value must be valid utf8
    fn from(value: Vec<&str>) -> Self {
        let value = value.into_iter();
        let value = value.map(|x| {
            let x: Vec<_> = OsString::from(x).encode_wide().collect();
            let x: &[u8] = cast_slice(x.deref());
            let x = Cow::from(x.to_owned());
            x
        });
        let value = value.collect();
        Self(value, Default::default())
    }
}

impl<'a> Deref for ItemIdList<'a> {
    type Target = Vec<Cow<'a, [u8]>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait BoundedDeref<T, R> {
    fn with_ref(self, f: impl FnOnce(&T) -> windows_core::Result<R>) -> windows_core::Result<R>;
}

impl<T, R> BoundedDeref<T, R> for *const T {
    fn with_ref(self, f: impl FnOnce(&T) -> windows_core::Result<R>) -> windows_core::Result<R> {
        let ptr = unsafe { self.as_ref() }.ok_or(E_POINTER)?;
        f(ptr)
    }
}
