/*
- `Rc<T>` is a reference-counted smart pointer that enables multiple ownership of a value, allowing
the value to be shared between different parts of your program while ensuring the value is cleaned
up once the last reference goes out of scope.

- `Rc<T>` works by keeping track of the number of references to the data it holds, automatically
deallocating the value when the reference count reaches zero.

- `Rc<T>` is only for single-threaded scenarios; it is marked as `!Send` and `!Sync`, meaning it
cannot be shared between threads. For multi-threaded environments, you would use `Arc<T>` (atomic
reference counting) instead.

- Cloning an `Rc<T>` is cheap, as it only increments the reference count. Dropping an `Rc<T>`
decrements the reference count, potentially deallocating the value if no more references exist.

- `Rc<T>` uses the `clone()` method to create additional references to the value. Each cloned
`Rc<T>` increments the reference count, and the value is only deallocated when
all references are dropped.
*/
use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;


struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>
}


pub struct MyRc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>
}


impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {value, ref_count: Cell::new(1)});
        MyRc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}


impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.ref_count.set(inner.ref_count.get() + 1);

        MyRc {
            inner: self.inner,
            _marker: PhantomData
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