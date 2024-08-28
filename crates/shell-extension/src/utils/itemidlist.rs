use crate::utils::alloc::alloc_com_ptr;
use crate::utils::item_id::ItemId;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::ptr;
use windows::Win32::UI::Shell::Common::ITEMIDLIST;

#[repr(transparent)]
#[derive(PartialEq, PartialOrd)]
pub struct ItemIdList(pub Vec<Box<[u8]>>);
impl ItemIdList {
    pub fn to_com_ptr(&self) -> windows::core::Result<*mut ITEMIDLIST> {
        let total_size = self.0.iter().map(|x| x.len() + 2).sum::<usize>() + 2;
        let memory = alloc_com_ptr(total_size)?;
        let mut next = memory.cast::<u8>();

        unsafe {
            for item_id in &self.0 {
                let size = item_id.len();
                let item_id = item_id.as_ptr();
                next.cast::<u16>().write_unaligned(size as u16 + 2);
                next = next.wrapping_byte_add(2);
                next.copy_from_nonoverlapping(item_id, size);
                next = next.wrapping_byte_add(size);
            }
            next.cast::<u16>().write_unaligned(0u16);
        }
        Ok(memory.cast())
    }
}

impl Debug for ItemIdList {
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

impl From<*const ITEMIDLIST> for ItemIdList {
    fn from(value: *const ITEMIDLIST) -> Self {
        let mut result = Self(vec![]);
        let mut next = value.cast::<u16>();
        loop {
            unsafe {
                let length = next.read_unaligned() as usize;
                if length == 0 {
                    break;
                }

                let entry = ptr::from_raw_parts::<ItemId>(next, length);
                let entry = entry.as_ref().unwrap();
                let entry = Box::from(&entry.content);
                result.0.push(entry);
                next = next.wrapping_byte_add(length);
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
