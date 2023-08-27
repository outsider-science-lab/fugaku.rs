use mpi_sys as ffi;
use ffi::{
  MPI_Comm,
  MPI_SUCCESS,
};
use crate::communicator::Communicator;
use mpi_common::ThreadLevel;

pub struct Universe {
  level: ThreadLevel,
}

pub fn initialize() -> anyhow::Result<Universe> {
  let level = mpi_common::initialize()?;
  Ok(Universe {
    level,
  })
}

pub fn initialize_thread(level: ThreadLevel) -> anyhow::Result<Universe> {
  let actual_level = mpi_common::initialize_thread(level)?;
  Ok(Universe {
    level: actual_level,
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
