/*
- `Send` is a marker trait that allows transferring ownership of types between threads.
If a type implements `Send`, it indicates that it can be safely moved to a different thread.

- Most primitive types in Rust, such as `i32` and `String`, automatically implement `Send`.
However, raw pointers (`*const T` or `*mut T`) do not implement `Send` by default since
they lack inherent thread safety.

- `Send` can be implemented manually for custom types using `unsafe impl Send`, but this should
be done with caution to ensure there are no risks of data races or unsound memory access.

- `Send` is automatically implemented for types that contain `Send` data, unless explicitly marked
otherwise.
*/
struct MySendType<T> {
    // raw pointer to T; raw pointers are neither `Send` nor `Sync` by default due to the risk
    // of unsafe memory access
    data: *mut T
}


impl<T> MySendType<T> {
    pub fn new(value: T) -> Self {
        // allocates value on the heap
        let boxed = Box::new(value);

        Self {
            // converts the `Box<T>` into a raw pointer
            data: Box::into_raw(boxed)
        }
    }


    pub fn get(&self) -> &T {
        unsafe { &*self.data }
    }


    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data }
    }
}

// manually implement `Send` because raw pointers do not implement `Send` by default
unsafe impl<T> Send for MySendType<T> {}


#[cfg(test)]
mod tests {
    use std::thread;
    use crate::send::MySendType;


    #[test]
    fn my_send_type() {
        let mut my_send_type = MySendType::new(String::from("Hello World!"));

        // `my_send_type` is moved to a new thread
        let handle = thread::spawn(move || {
            assert_eq!(*my_send_type.get(), String::from("Hello World!"));

            // modifies the value through the mutable reference
            *my_send_type.get_mut() = String::from("Hey Hey!");
            assert_eq!(*my_send_type.get(), String::from("Hey Hey!"));
        });

        // wait for the thread to finish
        handle.join().unwrap();
    }
}