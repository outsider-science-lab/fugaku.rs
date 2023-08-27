
pub fn malloc<T>() -> T {
  unsafe {
    std::mem::MaybeUninit::<T>::zeroed().assume_init()
  }
}

pub fn nullptr<T>() -> *mut T {
  std::ptr::null_mut()
}
