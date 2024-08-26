use crate::utils::item_id::ItemId;
use crate::utils::location::Location;
use crate::utils::location::Location::{Foreign, Local};
use std::alloc;
use std::alloc::alloc;
use std::fmt::Debug;
use std::mem::transmute;
use std::ops::Deref;
use std::ptr;
use std::ptr::slice_from_raw_parts_mut;
use std::vec::IntoIter;
use windows::Win32::System::Com::CoTaskMemAlloc;
use windows::Win32::UI::Shell::Common::ITEMIDLIST;

#[repr(transparent)]
#[derive(Debug)]
pub struct ItemIdList(Vec<Location<ItemId>>);
impl ItemIdList {
    fn to_com_ptr(self) -> Option<*mut ITEMIDLIST> {
        let total_size = self.0.iter().map(|x| x.size()).sum();
        let memory = unsafe { CoTaskMemAlloc(total_size) }.cast::<ITEMIDLIST>();
        if memory.is_null() {
            return None;
        }
        let mut next = memory.cast::<u8>();
        for item_id in self.0 {
            let size = item_id.size();
            let item_id = ptr::from_ref(item_id.deref()).cast::<u8>();
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
        let mut result = Self(vec![]);
        let mut next = value.cast::<u16>();
        loop {
            let length = unsafe { next.read() } as usize;
            let entry: *const ItemId = ptr::from_raw_parts(next, length);
            next = unsafe { next.add(length) };
            result.0.push(Foreign(unsafe { entry.as_ref().unwrap() }));
            if length == 0 {
                break;
            }
        }
        result
    }
}
impl From<IntoIter<Location<ItemId>>> for ItemIdList {
    fn from(value: IntoIter<Location<ItemId>>) -> Self {
        Self(
            value
                .map(|x| {
                    if let Location::Foreign(foreign_item_id) = x {
                        let size = foreign_item_id.size();
                        let foreign_item_id = ptr::from_ref(foreign_item_id).cast::<u8>();

                        let layout = alloc::Layout::from_size_align(size, 1).unwrap();

                        let local_item_id = unsafe { alloc(layout) };
                        unsafe { local_item_id.copy_from_nonoverlapping(foreign_item_id, size) };
                        let local_item_id = slice_from_raw_parts_mut(local_item_id, size);

                        let local_item_id = unsafe { Box::from_raw(local_item_id) };
                        Local(unsafe { transmute::<Box<[u8]>, Box<ItemId>>(local_item_id) })
                    } else {
                        x
                    }
                })
                .collect(),
        )
    }
}
impl Deref for ItemIdList {
    type Target = Vec<Location<ItemId>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test() {
    let meow: ItemIdList = vec![Local(Box::new("lol".as_bytes()))].into();
}
