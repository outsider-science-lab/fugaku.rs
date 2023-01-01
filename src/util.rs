
pub fn malloc<T>() -> T {
  return unsafe {
    std::mem::MaybeUninit::<T>::zeroed().assume_init()
  };
}
