mod ffi;

pub use ffi::MPI_Comm;

pub fn comm_world() -> ffi::MPI_Comm {
  unsafe {
    &mut ffi::ompi_mpi_comm_world as *mut ffi::ompi_predefined_communicator_t as ffi::MPI_Comm
  }
}

pub fn initialize() -> anyhow::Result<()> {
  let args: Vec<String> = std::env::args().collect();
  let mut argc = args.len() as i32;
  let mut argv: Vec<*mut u8> = Vec::new();
  for arg in &args {
    argv.push(arg.as_ptr() as *mut u8);
  }
  let mut argv_ptr = argv.as_mut_ptr();
  let r = unsafe {
    ffi::MPI_Init(&mut argc as *mut i32, &mut argv_ptr as *mut *mut *mut u8) as u32
  };
  match r {
    ffi::MPI_SUCCESS => Ok(()),
    _ => Err(anyhow::Error::msg(format!("[MPI_Init] Unknown code: {}", r))),
  }
}

pub fn finalize() -> anyhow::Result<()> {
  let r = unsafe {
    ffi::MPI_Finalize() as u32
  };
  match r {
    ffi::MPI_SUCCESS => Ok(()),
    _ => Err(anyhow::Error::msg(format!("[MPI_Finalize] Unknown code: {}", r))),
  }
}

pub fn comm_size(comm: ffi::MPI_Comm) -> anyhow::Result<usize> {
  let mut size: i32 = 0;
  let r = unsafe {
    ffi::MPI_Comm_size(comm, &mut size) as u32
  };
  match r {
    ffi::MPI_SUCCESS => Ok(size as usize),
    _ => Err(anyhow::Error::msg(format!("[MPI_Comm_size] Unknown code: {}", r))),
  }
}

pub fn comm_rank(comm: ffi::MPI_Comm) -> anyhow::Result<usize> {
  let mut rank: i32 = 0;
  let r = unsafe {
    ffi::MPI_Comm_rank(comm, &mut rank) as u32
  };
  match r {
    ffi::MPI_SUCCESS => Ok(rank as usize),
    _ => Err(anyhow::Error::msg(format!("[MPI_Comm_size] Unknown code: {}", r))),
  }
}
