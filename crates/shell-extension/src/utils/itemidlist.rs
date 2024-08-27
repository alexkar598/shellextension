use crate::utils::alloc::alloc_com_ptr;
use crate::utils::item_id::ItemId;
use std::fmt::Debug;
use std::ops::Deref;
use std::ptr;
use windows::Win32::UI::Shell::Common::ITEMIDLIST;

#[repr(transparent)]
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ItemIdList(pub Vec<Box<[u8]>>);
impl ItemIdList {
    pub fn to_com_ptr(&self) -> windows::core::Result<*mut ITEMIDLIST> {
        let total_size = self.0.iter().map(|x| x.len()).sum();
        let memory = alloc_com_ptr::<ITEMIDLIST>(total_size)?.as_mut_ptr();
        let mut next = memory.cast::<u8>();
        for item_id in &self.0 {
            let size = item_id.len();
            let item_id = item_id.as_ptr();
            unsafe {
                next.copy_from_nonoverlapping(item_id, size);
            }
            next = next.wrapping_add(size);
        }
        Ok(memory)
    }
}

impl From<*const ITEMIDLIST> for ItemIdList {
    fn from(value: *const ITEMIDLIST) -> Self {
        let mut result = Self(vec![]);
        let mut next = value.cast::<u8>();
        loop {
            unsafe {
                let length = next.cast::<u16>().read_unaligned() as usize;
                if length == 0 {
                    break;
                }

                let entry = ptr::from_raw_parts::<ItemId>(next, length);
                let entry = entry.as_ref().unwrap();
                let entry = Box::from(&entry.content);
                result.0.push(entry);
                next = next.wrapping_add(length);
            }
        }
        result
    }
}
impl From<Vec<Box<[u8]>>> for ItemIdList {
    fn from(value: Vec<Box<[u8]>>) -> Self {
        Self(value)
    }
}

impl Deref for ItemIdList {
    type Target = Vec<Box<[u8]>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test() {}
