use std::ffi::{
  c_int,
  c_uint,
};

use fujitsu_mpi_sys as ffi;
use ffi::{
  MPI_Comm,
  MPI_SUCCESS,
};
use crate::communicator::Communicator;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreadLevel {
  Single,
  Funneled,
  Serialized,
  Multiple,
}

impl ThreadLevel {
  fn to_ffi(&self) -> c_int {
    match self {
      &Self::Single => ffi::MPI_THREAD_SINGLE as c_int,
      &Self::Funneled => ffi::MPI_THREAD_FUNNELED as c_int,
      &Self::Serialized => ffi::MPI_THREAD_SERIALIZED as c_int,
      &Self::Multiple => ffi::MPI_THREAD_MULTIPLE as c_int,
    }
  }
  fn from_ffi(level: c_int) -> anyhow::Result<Self> {
    match level as c_uint {
      ffi::MPI_THREAD_SINGLE => Ok(Self::Single),
      ffi::MPI_THREAD_FUNNELED => Ok(Self::Funneled),
      ffi::MPI_THREAD_SERIALIZED => Ok(Self::Serialized),
      ffi::MPI_THREAD_MULTIPLE => Ok(Self::Multiple),
      level => Err(anyhow::Error::msg(format!("Unknwon thread level: {}", level)))
    }
  }
}

pub struct Universe {
  level: ThreadLevel,
}

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

pub fn initialize() -> anyhow::Result<Universe> {
  if initialized()? {
    return Err(anyhow::Error::msg("MPI: Already initialized."))
  }
  with_args(|argc, argv| {
    let r = unsafe {
      ffi::MPI_Init(argc, argv) as u32
    };
    match r {
      MPI_SUCCESS => Ok(Universe {
        level: ThreadLevel::Single,
      }),
      _ => Err(anyhow::Error::msg(format!("[MPI_Init] Unknown code: {}", r))),
    }  
  })
}

pub fn initialize_thread(level: ThreadLevel) -> anyhow::Result<Universe> {
  if initialized()? {
    return Err(anyhow::Error::msg("MPI: Already initialized."))
  }
  with_args(|argc, argv| {
    let mut provided = 0;
    // https://rookiehpc.github.io/mpi/docs/mpi_init_thread/
    let r = unsafe {
      ffi::MPI_Init_thread(argc, argv, level.to_ffi(), &mut provided) as u32
    };
    match r {
      MPI_SUCCESS => Ok(Universe {
        level: ThreadLevel::from_ffi(provided)?,
      }),
      _ => Err(anyhow::Error::msg(format!("[MPI_Init] Unknown code: {}", r))),
    }  
  })
}

impl Universe {
  pub fn level(&self) -> ThreadLevel {
    self.level
  }
  pub fn world(&mut self) -> Communicator {
    Communicator::new(unsafe {
      &mut ffi::ompi_mpi_comm_world as *mut ffi::ompi_predefined_communicator_t as MPI_Comm
    })
  }
}

impl Drop for Universe {
  fn drop(&mut self) {
    let r = unsafe {
      ffi::MPI_Finalize() as u32
    };
    if r != MPI_SUCCESS {
      // TODO: Error handling?
    }
  }
}
