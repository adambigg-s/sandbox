/// this is by no means actually a thread-safe pointer, but for this application
/// it doesn't matter and it is so much faster than the thread-safe alternatives
pub struct RawPtrMut<T> {
    ptr: *mut T,
}

impl<T> RawPtrMut<T> {
    pub fn build(ptr: *mut T) -> Self {
        RawPtrMut { ptr }
    }

    pub fn deref(&mut self) -> &mut T {
        unsafe { &mut (*self.ptr) }
    }
}

unsafe impl<T> Send for RawPtrMut<T> {}

unsafe impl<T> Sync for RawPtrMut<T> {}

impl<T> Copy for RawPtrMut<T> {}

impl<T> Clone for RawPtrMut<T> {
    fn clone(&self) -> Self {
        *self
    }
}
