use std::alloc::dealloc;
use std::marker::PhantomPinned;
use std::mem::ManuallyDrop;

#[repr(C, packed(1))]
pub struct SHITEMID {
    pub cb: u16,
    pub abID: ManuallyDrop<[u8]>,
}

#[repr(C, packed(1))]
pub struct ITEMIDLIST {
    pub mkid: *mut SHITEMID,
}

impl Into<Vec<&SHITEMID>> for ITEMIDLIST {
    fn into(self) -> [*mut SHITEMID] {
        todo!()
    }
}