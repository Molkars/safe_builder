use std::ops::{Deref, DerefMut};

pub struct Absent;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Present<T>(pub T);

impl<T: PartialEq> PartialEq<T> for Present<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for Present<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T> AsRef<T> for Present<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Present<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Present<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}