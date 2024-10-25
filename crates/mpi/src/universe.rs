use log::error;
use mpi_sys as ffi;
use ffi::{
  MPI_Comm,
  MPI_SUCCESS,
};
use crate::communicator::Communicator;
use mpi_common::ThreadLevel;
pub use mpi_common::initialized;

pub struct Universe {
  level: ThreadLevel,
}

pub fn initialize() -> anyhow::Result<Universe> {
  Ok(Universe {
    level: mpi_common::initialize()?,
  })
}

pub fn initialize_thread(level: ThreadLevel) -> anyhow::Result<Universe> {
  Ok(Universe {
    level: mpi_common::initialize_thread(level)?,
  })
}

impl Universe {
  pub fn level(&self) -> ThreadLevel {
    self.level
  }

  pub fn world(&mut self) -> Communicator {
    Communicator::new(&raw mut ffi::ompi_mpi_comm_world as MPI_Comm)
  }
}

impl Drop for Universe {
  fn drop(&mut self) {
    let r = unsafe {
      ffi::MPI_Finalize() as u32
    };
    if r != MPI_SUCCESS {
      error!("Failed to execute MPI_Finalize, code = {}", r);
    }
  }
}
