use std::intrinsics::transmute;
use windows_core::{OutRef, Type};

pub trait OutRefExtension<T: Type<T>> {
    fn into_ptr(self) -> *mut T::Abi;
}
impl<'a, T: Type<T>> OutRefExtension<T> for OutRef<'a, T> {
    fn into_ptr(self) -> *mut T::Abi {
        unsafe {transmute::<Self, *mut T::Abi>(self)}
    }
}