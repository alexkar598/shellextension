use std::ops::Deref;

#[derive(Debug)]
pub enum Location<T: 'static + ?Sized> {
    Foreign(&'static T),
    Local(Box<T>),
}

impl<T: 'static + ?Sized> Deref for Location<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Location::Foreign(item) => item,
            Location::Local(item) => item,
        }
    }
}
