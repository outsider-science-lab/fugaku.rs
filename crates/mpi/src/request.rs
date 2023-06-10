// Doc:
// - https://rookiehpc.github.io/mpi/docs/mpi_request/

use mpi_sys as ffi;
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

  pub fn test_all(requests: &[&mut Request]) -> anyhow::Result<Vec<bool>> {
    let mut flags = vec![0; requests.len()];
    let mut inners = requests.iter().map(|it| it.inner).collect::<Vec<_>>();
    let r = unsafe {
      ffi::MPI_Testall(
        requests.len() as i32,
        inners.as_mut_ptr(),
        flags.as_mut_ptr(),
        std::ptr::null_mut(),
      ) as u32
    };
    match r {
      MPI_SUCCESS => Ok(flags.iter().map(|it| *it != 0).collect()),
      _ => Err(anyhow::Error::msg(format!("[MPI_Wait] Unknown code: {}", r))),
    }
  }
}
