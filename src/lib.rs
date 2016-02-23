use std::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! defer {
    ($e:expr) => {
        let _guard = $crate::guard((), |_| $e);
    }
}

/// `Guard` is a scope guard that may own a protected value.
///
/// If you place a guard value in a local variable, its destructor will
/// run regardless how you leave the function — regular return or panic
/// (barring truly abnormal incidents).
///
/// The guard's closure will be called with a mut ref to the held value
/// in the destructor. It's called only once.
pub struct Guard<T, F>
    where F: FnMut(&mut T)
{
    __dropfn: F,
    __value: T,
}

/// Create a new `Guard` owning `v` and with deferred closure `dropfn`.
pub fn guard<T, F>(v: T, dropfn: F) -> Guard<T, F>
    where F: FnMut(&mut T)
{
    Guard{__value: v, __dropfn: dropfn}
}

impl<T, F> Deref for Guard<T, F>
    where F: FnMut(&mut T)
{
    type Target = T;
    fn deref(&self) -> &T
    {
        &self.__value
    }

}

impl<T, F> DerefMut for Guard<T, F>
    where F: FnMut(&mut T)
{
    fn deref_mut(&mut self) -> &mut T
    {
        &mut self.__value
    }
}

impl<T, F> Drop for Guard<T, F>
    where F: FnMut(&mut T)
{
    fn drop(&mut self) {
        (self.__dropfn)(&mut self.__value)
    }
}

#[test]
fn test_defer() {
    use std::cell::Cell;

    let drops = Cell::new(0);
    defer!(drops.set(1000));
    assert_eq!(drops.get(), 0);
}

