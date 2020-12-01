use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

struct Pool<T> {
    items: RefCell<Vec<T>>,
}

impl<T: PoolItem> Pool<T> {
    fn new() -> Self {
        Self {
            items: RefCell::new(Vec::new()),
        }
    }

    fn get(&self) -> PoolGuard<T> {
        let item = match self.items.borrow_mut().pop() {
            Some(item) => item,
            None => T::new(),
        };
        PoolGuard {
            inner: Some(item),
            items: &self.items,
        }
    }
}

trait PoolItem {
    fn new() -> Self;
    fn reset(&mut self);
}

struct PoolGuard<'a, T: PoolItem> {
    inner: Option<T>,
    items: &'a RefCell<Vec<T>>,
}

impl<T: PoolItem> Drop for PoolGuard<'_, T> {
    fn drop(&mut self) {
        let mut item = self.inner.take().unwrap();
        item.reset();
        self.items.borrow_mut().push(item);
    }
}

impl<T: PoolItem> Deref for PoolGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<T: PoolItem> DerefMut for PoolGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

#[cfg(test)]
mod pool_tests {
    use super::*;

    struct Awesome(usize);

    impl Awesome {
        fn get(&self) -> usize {
            self.0
        }

        fn inc(&mut self) {
            self.0 += 1
        }
    }

    impl PoolItem for Awesome {
        fn new() -> Self {
            Self(0)
        }
        fn reset(&mut self) {
            self.0 = 0
        }
    }

    #[test]
    fn it_works() {
        let pool: Pool<Awesome> = Pool::new();
        let mut item = pool.get();

        item.inc();
        assert_eq!(item.get(), 1);
        drop(item);
        let new_item = pool.get();
        assert_eq!(new_item.get(), 1);
    }
}
