/*
- `RefCell<T>` provides interior mutability, allowing mutable access to data even when accessed
through an immutable reference (`&self`), but it does so with runtime borrowing checks.

- Unlike `Cell<T>`, which works with `Copy` types, `RefCell<T>` allows borrowing references
to its inner value. It enforces Rust’s borrowing rules (one mutable or multiple immutable borrows)
at runtime rather than compile time.

- `RefCell<T>` uses `borrow()` for immutable access and `borrow_mut()` for mutable access.
These methods track borrows and panic if the borrowing rules are violated (e.g., if you try to
borrow mutably while there’s an active immutable borrow).

- Like `Cell<T>`, `RefCell<T>` is marked as `!Sync` and cannot be shared between threads due to
the potential for unsynchronized mutation.

- At its core, `RefCell<T>` leverages `UnsafeCell<T>` to provide safe interior mutability while
enforcing borrowing rules dynamically.
*/
use std::ops::{Deref, DerefMut};
use std::cell::{Cell, UnsafeCell};

pub struct MyRefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>
}

impl<T> MyRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared)
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref {refcell: self})
            },
            RefState::Shared(count) => {
                self.state.set(RefState::Shared(count + 1));
                Some(Ref {refcell: self})
            },
            RefState::Exclusive => None
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(RefMut {refcell: self})
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive
}

struct Ref<'refcell, T> {
    refcell: &'refcell MyRefCell<T>
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            },
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1))
            }
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { & *self.refcell.value.get() }
    }
}

struct RefMut<'refcell, T> {
    refcell: &'refcell MyRefCell<T>
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { & *self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::{MyRefCell, RefState};

    #[test]
    fn my_ref_cell_new() {
        let ref_cell = MyRefCell::new(String::from("MyRefCell"));

        assert_eq!(ref_cell.borrow().unwrap().as_str(), "MyRefCell");
        assert_eq!(ref_cell.state.get(), RefState::Unshared);
    }

    #[test]
    fn my_ref_cell_borrow() {
        let ref_cell = MyRefCell::new(String::from("MyRefCell"));

        let ref_cell_borrow_1 = ref_cell.borrow().unwrap();
        let ref_cell_borrow_2 = ref_cell.borrow().unwrap();

        {
            let ref_cell_borrow_3 = ref_cell.borrow().unwrap();
            assert_eq!(ref_cell.state.get(), RefState::Shared(3));
        }

        let ref_cell_borrow_mut = ref_cell.borrow_mut();

        assert_eq!(ref_cell_borrow_1.as_str(), "MyRefCell");
        assert_eq!(ref_cell_borrow_2.as_str(), "MyRefCell");
        assert_eq!(ref_cell.state.get(), RefState::Shared(2));
        assert!(ref_cell_borrow_mut.is_none());
    }

    #[test]
    fn my_ref_cell_borrow_mut() {
        let ref_cell = MyRefCell::new(String::from("MyRefCell"));

        let ref_cell_borrow_mut = ref_cell.borrow_mut();
        let ref_cell_borrow_1 = ref_cell.borrow();
        let ref_cell_borrow_2 = ref_cell.borrow();

        assert_eq!(ref_cell.state.get(), RefState::Exclusive);
        assert!(ref_cell_borrow_1.is_none());
        assert!(ref_cell_borrow_2.is_none());
    }
}