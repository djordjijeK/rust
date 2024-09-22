/*
- `Cell<T>` allows interior mutability, enabling mutation of data through `get` and `set` methods
even when accessed via an immutable reference (`&self`). This only works in single-threaded
contexts.

- Normally, Rust requires a mutable reference (`&mut`) for mutation, but `Cell<T>` bypasses this
by using `UnsafeCell<T>` internally, which allows safe mutation through an otherwise
immutable reference.

- `Cell<T>` is marked as `!Sync`, meaning it cannot be shared across threads due to its ability
to mutate through immutable references, which breaks Rust's thread safety guarantees.

- `Cell<T>` requires `T` to implement the `Copy` trait for the `get` method, as it avoids borrowing
the inner value. Instead, it returns a copy to prevent issues with Rust's borrowing rules.
*/
use std::cell::UnsafeCell;

pub struct MyCell<T> {
    value: UnsafeCell<T>
}

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value)
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::MyCell;

    #[test]
    fn my_cell_new() {
        let cell = MyCell::new(10);
        assert_eq!(cell.get(), 10);
    }

    #[test]
    fn my_cell_set_and_get() {
        let cell = MyCell::new(0);
        let cell_ref = &cell;

        assert_eq!(cell.get(), 0);

        cell_ref.set(42);
        assert_eq!(cell.get(), 42);

        cell.set(100);
        assert_eq!(cell_ref.get(), 100);
    }
}
