use std::cmp::max;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomPinned;
use std::ops::Deref;
use std::ptr::slice_from_raw_parts;

#[repr(C, packed(1))]
pub struct ItemId {
    _pin: PhantomPinned,
    cb: u16,
    abID: [u8],
}

impl ItemId {
    pub fn content(&self) -> Option<&[u8]> {
        let content = slice_from_raw_parts(self.abID.as_ptr(), self.size() - 2);
        unsafe { content.as_ref() }
    }
    pub fn content_mut(&mut self) -> Option<&mut [u8]> {
        let content = slice_from_raw_parts(self.abID.as_ptr(), self.size() - 2).cast_mut();
        unsafe { content.as_mut() }
    }
    pub fn size(&self) -> usize {
        max(self.cb as usize, 2)
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
