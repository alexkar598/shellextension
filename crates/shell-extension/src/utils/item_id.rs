use std::fmt::{Debug, Formatter};

#[repr(C, packed(1))]
pub struct ItemId {
    pub size: u16,
    pub content: [u8],
}

impl Debug for ItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}
