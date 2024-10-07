use std::cell::Cell;
use std::ops::Deref;
use std::ptr::NonNull;


struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>
}


pub struct MyRc<T> {
    pub inner: NonNull<RcInner<T>>
}


impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {value, ref_count: Cell::new(1)});
        MyRc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) }
        }
    }
}


impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.ref_count.set(inner.ref_count.get() + 1);

        MyRc {
            inner: self.inner
        }
    }
}


impl<T> Deref for MyRc<T> {
    type Target = T;


    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}


impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let ref_count = unsafe { self.inner.as_ref() }.ref_count.get();

        if ref_count == 1 {
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            unsafe { self.inner.as_ref() }.ref_count.set(ref_count - 1);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::rc::MyRc;


    #[test]
    fn my_rc_new() {
        let my_rc = MyRc::new(String::from("Hello World!"));

        assert_eq!(unsafe { my_rc.inner.as_ref() }.ref_count.get(), 1);
        assert_eq!(*my_rc, String::from("Hello World!"));
    }


    #[test]
    fn my_rc_clone() {
        let my_rc = MyRc::new(String::from("Hello World!"));

        {
            let my_rc_first_clone = my_rc.clone();
            let my_rc_second_clone = my_rc_first_clone.clone();

            assert_eq!(unsafe { my_rc_first_clone.inner.as_ref() }.ref_count.get(), 3);
            assert_eq!(*my_rc_second_clone, String::from("Hello World!"));
        }

        assert_eq!(unsafe { my_rc.inner.as_ref() }.ref_count.get(), 1);
        assert_eq!(*my_rc, String::from("Hello World!"));
    }
}