// Doc:
// - https://rookiehpc.github.io/mpi/docs/mpi_request/index.html

use fujitsu_mpi_sys as ffi;
use ffi::{
  MPI_Request,
  MPI_Status,
  MPI_SUCCESS,
};
use crate::util::malloc;

pub struct Request {
  inner: MPI_Request,
}

impl Request {
  pub(crate) fn new(inner: MPI_Request) -> Self {
    Self {
      inner,
    }
  }

  pub fn test(&mut self) -> anyhow::Result<bool> {
    let mut status: MPI_Status = malloc();
    let mut ready = 0;
    let r = unsafe {
        ffi::MPI_Test(
          &mut self.inner,
          &mut ready,
          &mut status,
        ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(ready != 0),
      _ => Err(anyhow::Error::msg(format!("[MPI_Test] Unknown code: {}", r))),
    }
  }

  pub fn wait(&mut self) -> anyhow::Result<()> {
    let mut status: MPI_Status = malloc();
    let r = unsafe {
        ffi::MPI_Wait(
          &mut self.inner,
          &mut status,
        ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Wait] Unknown code: {}", r))),
    }
  }
}
