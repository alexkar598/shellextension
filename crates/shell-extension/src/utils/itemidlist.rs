use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomPinned;
use std::ops::{AddAssign, Deref};
use std::ptr;
use std::ptr::{null, slice_from_raw_parts};
use windows::core::Result;
use windows::Win32::System::Com::CoTaskMemAlloc;
use windows::Win32::UI::Shell::Common::ITEMIDLIST;
use crate::utils::HRESULT;

enum Location<T : 'static + ?Sized> {
    Foreign(&'static T),
    Local(Box<T>)
}

#[repr(C, packed(1))]
pub struct ItemId {
    _pin: PhantomPinned,
    pub cb: u16,
    pub abID: [u8],
}

impl ItemId {
    fn content(&self) -> Option<&[u8]> {
        let content = slice_from_raw_parts(self.abID.as_ptr(), self.cb.saturating_sub(2) as usize);
        unsafe { content.as_ref() }
    }
}

impl Debug for ItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let content = self.content();
        match content {
            None => write!(f, "None()"),
            Some(bytes) => {
                let text = String::from_utf8_lossy(bytes).replace("\0", "");
                write!(f, "Text({text})")
            }
        }
    }
}

#[repr(transparent)]
pub struct ItemIdList (Vec<Location<ItemId>>);

impl ItemIdList {
    fn to_com_ptr(self) -> Option<*mut ITEMIDLIST> {
        let total_size = self.0.iter().fold(0, |x, item| x + item.cb as usize) + 2;
        let memory = unsafe {CoTaskMemAlloc(total_size)}.cast::<ITEMIDLIST>();
        if memory.is_null() {
            return None;
        }
        let mut next = memory.cast::<u8>();
        for item_id in self.0 {
            let size = item_id.cb as usize;
            let item_id = ptr::from_ref(item_id).cast::<u8>();
            unsafe {
                next.copy_from_nonoverlapping(item_id, size);
            }
            next = next.wrapping_add(size);
        }
        Some(memory)
    }
}
impl From<*const ITEMIDLIST> for ItemIdList {
    fn from(value: *const ITEMIDLIST) -> Self {
        let mut result: Vec<&'static ItemId> = vec!();
        let mut next = value.cast::<u16>();
        loop {
            let length = unsafe {next.read()} as usize;
            let entry: *const ItemId = ptr::from_raw_parts(next, length);
            next = unsafe {next.add(length)};
            result.push(unsafe {entry.as_ref().unwrap()});
            if length == 0 {
                break
            }
        };
        Self(result)
    }
}
impl Deref for ItemIdList {
    type Target = Vec<&'static ItemId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test() {
    let meow: ItemIdList = null::<ITEMIDLIST>().into();
}