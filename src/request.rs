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
