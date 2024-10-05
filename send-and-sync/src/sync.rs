/*
- `Sync` is a marker trait that allows shared references (`&T`) of a type to be accessed by multiple
threads concurrently. If a type is `Sync`, it implies the type can be safely shared across threads.

- Types like `Arc<T>` and `Mutex<T>` are `Sync`, meaning they are designed to be shared between
threads and provide the necessary synchronization internally to prevent race conditions.

- Unlike `Send`, `Sync` implies that the type can be referenced (not moved) across threads,
meaning the type is safe to be accessed concurrently from multiple threads.

- Most types in Rust implement `Sync` if their data members are also `Sync`. However, types
like `Cell<T>` and `RefCell<T>` are marked `!Sync` because their internal mutability could lead
to unsynchronized mutation if shared between threads.

- Both `Send` and `Sync` traits are marker traits, meaning they don’t contain methods but instead
serve as guarantees to the Rust compiler about thread safety properties.
*/
use std::sync::Mutex;


struct MyCounter {
    // mutex provides mutual exclusion to protect access to the count value
    // only one thread can access the value at a time
    count: Mutex<i32>
}


impl MyCounter {
    pub fn new() -> Self {
        MyCounter {
            count: Mutex::new(0)
        }
    }


    pub fn increment(&self) {
        // the lock() method will block until the lock is acquired
        let mut count = self.count.lock().unwrap();

        // by dereferencing the MutexGuard (`count`), we access the inner `i32` and increment it
        *count += 1;
    }


    pub fn get(&self) -> i32 {
        // ensures that no other thread can mutate `count` while we are reading it
        let count = self.count.lock().unwrap();
        *count
    }
}


#[cfg(test)]
mod tests {
    use std::thread;
    use std::sync::Arc;
    use crate::sync::MyCounter;


    #[test]
    fn my_counter() {
        // `Arc` is used to allow multiple threads to have ownership of the `MyCounter` instance
        let counter = Arc::new(MyCounter::new());

        let mut handles = vec![];
        for _ in 0..100 {
            // `clone()` increments the reference count of the `Arc`
            let counter_ref = counter.clone();

            handles.push(
                thread::spawn(move || {
                    // `move` is needed because `counter_ref` is captured by the closure and
                    // transferred to the thread
                    counter_ref.increment();
                })
            );
        }

        // join threads to ensure all increments are complete before the assertion
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 100);
    }
}
