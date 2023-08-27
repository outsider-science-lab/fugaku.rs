use std::ffi::{
  c_int,
  c_uint,
};

use mpi_sys as ffi;

// https://rookiehpc.github.io/mpi/docs/mpi_thread_single/
// https://rookiehpc.github.io/mpi/docs/mpi_thread_funneled/
// https://rookiehpc.github.io/mpi/docs/mpi_thread_serialized/
// https://rookiehpc.github.io/mpi/docs/mpi_thread_multiple/
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreadLevel {
  Single,
  Funneled,
  Serialized,
  Multiple,
}

impl ThreadLevel {
  pub fn to_ffi(&self) -> c_int {
    match self {
      &Self::Single => ffi::MPI_THREAD_SINGLE as c_int,
      &Self::Funneled => ffi::MPI_THREAD_FUNNELED as c_int,
      &Self::Serialized => ffi::MPI_THREAD_SERIALIZED as c_int,
      &Self::Multiple => ffi::MPI_THREAD_MULTIPLE as c_int,
    }
  }
  pub fn from_ffi(level: c_int) -> anyhow::Result<Self> {
    match level as c_uint {
      ffi::MPI_THREAD_SINGLE => Ok(Self::Single),
      ffi::MPI_THREAD_FUNNELED => Ok(Self::Funneled),
      ffi::MPI_THREAD_SERIALIZED => Ok(Self::Serialized),
      ffi::MPI_THREAD_MULTIPLE => Ok(Self::Multiple),
      level => Err(anyhow::Error::msg(format!("Unknwon thread level: {}", level)))
    }
  }
}
