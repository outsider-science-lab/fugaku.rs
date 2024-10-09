
pub unsafe fn malloc<T>() -> T {
  std::mem::MaybeUninit::<T>::zeroed().assume_init()
}

pub fn nullptr<T>() -> *mut T {
  std::ptr::null_mut()
}

pub unsafe fn as_void_ptr<T>(buff: &[T]) -> *const std::os::raw::c_void {
  buff.as_ptr() as *const std::os::raw::c_void
}

pub unsafe fn as_mut_void_ptr<T>(buff: &mut [T]) -> *mut std::os::raw::c_void {
  buff.as_mut_ptr() as *mut std::os::raw::c_void
}
