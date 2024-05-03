use mpi_sys as ffi;
use crate::thread_level::ThreadLevel;
use ffi::MPI_SUCCESS;

fn with_args<R, F>(closure: F) -> R
  where F: FnOnce(*mut i32, *mut *mut *mut u8) -> R
{
  let args: Vec<String> = std::env::args().collect();
  let mut argc = args.len() as i32;
  let mut argv: Vec<*mut u8> = Vec::new();
  for arg in &args {
    argv.push(arg.as_ptr() as *mut u8);
  }
  let mut argv_ptr = argv.as_mut_ptr();
  closure(&mut argc as *mut i32, &mut argv_ptr as *mut *mut *mut u8)
}

pub fn initialized() -> anyhow::Result<bool> {
  let mut ready = 0;
  let r = unsafe {
    ffi::MPI_Initialized(&mut ready) as u32
  };
  match r {
    MPI_SUCCESS => Ok(ready != 0),
    _ => Err(anyhow::Error::msg(format!("[MPI_Initialized] Unknown code: {}", r))),
  }  
}

pub fn initialize() -> anyhow::Result<ThreadLevel> {
  if initialized()? {
    return Err(anyhow::Error::msg("MPI: Already initialized."))
  }
  with_args(|argc, argv| {
    let r = unsafe {
      ffi::MPI_Init(argc, argv) as u32
    };
    match r {
      MPI_SUCCESS => Ok(ThreadLevel::Single),
      _ => Err(anyhow::Error::msg(format!("[MPI_Init] Unknown code: {}", r))),
    }  
  })
}

pub fn initialize_thread(required: ThreadLevel) -> anyhow::Result<ThreadLevel> {
  if initialized()? {
    return Err(anyhow::Error::msg("MPI: Already initialized."))
  }
  with_args(|argc, argv| {
    let mut provided = 0;
    // https://rookiehpc.github.io/mpi/docs/mpi_init_thread/
    let r = unsafe {
      let required = required.to_ffi();
      ffi::MPI_Init_thread(argc, argv, required, &mut provided) as u32
    };
    match r {
      MPI_SUCCESS => Ok(ThreadLevel::from_ffi(provided)?),
      _ => Err(anyhow::Error::msg(format!("[MPI_Init_thread] Unknown code: {}", r))),
    }  
  })
}
