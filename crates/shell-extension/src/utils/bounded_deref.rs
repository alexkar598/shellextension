use windows_core::imp::E_POINTER;

pub trait BoundedDeref<T, R> {
    fn with_ref(self, f: impl FnOnce(&T) -> windows_core::Result<R>) -> windows_core::Result<R>;
}
pub trait BoundedDerefMut<T, R> {
    fn with_mut_ref(
        self,
        f: impl FnOnce(&mut T) -> windows_core::Result<R>,
    ) -> windows_core::Result<R>;
}

impl<T, R> BoundedDeref<T, R> for *const T {
    fn with_ref(self, f: impl FnOnce(&T) -> windows_core::Result<R>) -> windows_core::Result<R> {
        let ptr = unsafe { self.as_ref() }.ok_or(E_POINTER)?;
        f(ptr)
    }
}
impl<T, R> BoundedDerefMut<T, R> for *mut T {
    fn with_mut_ref(
        self,
        f: impl FnOnce(&mut T) -> windows_core::Result<R>,
    ) -> windows_core::Result<R> {
        let ptr = unsafe { self.as_mut() }.ok_or(E_POINTER)?;
        f(ptr)
    }
}
